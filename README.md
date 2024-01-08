# Brief

使用netx和yew编写了blog，参考这个[视频](https://www.bilibili.com/video/BV1pY4y1Z7jR/?spm_id_from=333.788&vd_source=0cc0401ee122346d6680e90658b0ed1a)。

效果如下：

普通用户

![](https://moe.photo/images/2024/01/08/1.png)

管理员

![](https://moe.photo/images/2024/01/08/2.png)

添加文章

![](https://moe.photo/images/2024/01/08/3.png)

等等。。。。

# Usage

## 1

在server目录下操作：

- 将我们的程序变成wasm程序：`rustup target add wasm32-unknown-unknown`

- 查看安装了那些target： `rustup show`
- 安装wasm编译工具：`cargo install trunk`

启动：

- server：`cargo run`，在12345端口
- client：`trunk serve`，在8080端口

## 2

错误处理

## 3

获取所有文章的API。使用PostgreSQL。建库，建表。

记得开启postgresql服务：`sudo service postgresql start`

**表结构**

| id(自增)(int) | title(varchar) | context(text) | date(CURRENT_DATE)(date) |
| ------------- | -------------- | ------------- | ------------------------ |
| 1             |                |               |                          |
| ...           |                |               |                          |

```sql
CREATE TABLE IF NOT EXISTS articles(
   id SERIAL PRIMARY KEY,
   title VARCHAR(256) NOT NULL,
   content TEXT NOT NULL,
   date DATE DEFAULT CURRENT_DATE
);
```

![](https://moe.photo/images/2024/01/08/image-20240105022600845.png)

![](https://moe.photo/images/2024/01/08/image-20240105022636714.png)

**读取文章**

从数据库读取，返回json

- 路由设计:GET/articles(获取所有文章)
- 用到的库:
  - SQLx：数据库驱动，用于连接数据库
  - chrono：时间白期库，我们的表里面有一个date类型的字段，用这个库来处理
  - serde和serde json：序列化/反序列化库，用于处理JSON
  - dotenvy：读取.env文件并把里面的键值对加载进环境变量

- 我们还要为sqlx:.Error实现From trait，以便转为我们自定义的错误类型，方便错误处理

效果：

![](https://moe.photo/images/2024/01/08/image-20240105031717674.png)



## 4

新增文章：解析传入的JSON，提取数据，在数据库中插入数据

- 路由设计: POST /article
- 用户要提交上来的数据只有标题和文章内容，操作成功就返回成功的消息，否则就返回错误信息
- 我们要把Article 结构体的id和date字段改成 Option类型的，并修改以前写的代码

修改文章：在传入的JSON数据中获取ID，在数据库中更新数据

- 路由设计:PUT/article
- 和新增文章差不多，只不过用户需要提供要修改的文章的ID

## 5

- 搜索文章
  - 路由设计： GET/article/search/{keyword}
  - 简单的搜索功能：title/content中包含keyword
- 查看文章
  - 预览
  - 详细信息（单篇）
  - 路由设计：GET/article/{id}(单篇), GET/articles(预览)
  - 结构体：ArticlePreview struct (id, title, date)
- 改写路由（不适用宏）（可选）

## 6

使用Github授权登录（概念)，设置数据库(建表)：

- 请求用户的Github标识（跳转到Github的授权页面）：`GET https://github.com/login/oauth/authorize?client_id=xxx(?state=xxx)(CSRF)`。

- 用户用Github 登录后，被重定向回我们的站点，Github在URL 中附上code和(state)前端向后端发起一个请求，把code和state给后端，code的有效期为10分钟，并且是一次性的，不能重复使用。

- 后端拿到code和(state)之后，(对state进行验证)，向Github获取access_token(获取后code失效)：`POST https://github.com/login/oauth/access_token`。参数: `client_id`，`client_secret`，`code`。参数也是以?a=b&c=d...的形式附在URL上面的。可以把这个`access_token`存到cookie里，这样用户就无需重复登录了。

- 后端用access_token请求Github的API，获取用户信息，并把它们存进数据庭里，`Authorization: Bearer OAUTH-TOKEN (HTTP标头)`，`GET https://api.github.com/user`。

建表

1. 进入数据库：`sudo -u postgres psql`
2. `\l`查看所有的数据库，`\c`选择使用数据库，`\d`查看所有表：我们输入`\c blog`，选择blog数据库
3. 输入下面的指令建表

```sql
CREATE TABLE IF NOT EXISTS users(
   id INT4 PRIMARY KEY,
   name VARCHAR(256) NOT NULL,
   avatar_url varchar(255) NOT NULL
);
```

暂时为这三个字段，以后有需要再添加：

| id(请求GitHub的API时返回的) | name(Github提供) | avatar_url(GitHub提供) |
| --------------------------- | ---------------- | ---------------------- |
| 11616515616                 | abcd             | https://.....          |
| ....                        | ....             | ....                   |

## 7

编写GitHub后端登录API：

路由设计： `POST /usr/login(application/json)`。示例：`{"code": "xxx"}`

## 8

使用中间件实现身份认证：中间件概念，实现身份认证

- 请求/响应在到达handler之前先经过中间件
- 中间件可以修改请求/响应
- 应用：解压请求与压缩响应（压缩算法:gzip...)，给响应添加HTTP headers.....身份验证

![](https://moe.photo/images/2024/01/08/image-20240105230402797.png)

ntex中的中间件：

- Middleware trait
- 定义了一个service factory(用于产生service的东西)
  - 接口(可以通过调用这个trait的create方法产生一个service)
  - 关于service:
    - Service是一个async的函数，它接收请求，返回响应，类似这样：`async fn(Request) ->Result<Response, Error>`，但是它与hand不同，handler有更多的功能
    - Service trait (call方法)(返回Future)
  - 中间件会在接收到请求/响应的时候运行里面的service

实现

- struct CheckLogin{db_pool, admin}(是否需要管理员权限)(impl Middleware)(暴露给用户使用)
- strucct CheckLoginMiddleware< S>{db_pool,admin,service:S}(impl Service)(实际的service,由CheckLogin的create方法产生)(在service的fall方法里身份认证的逻辑)

当理解了，代码就不敲了

## 9

使用更简单、更灵活的方式实现身份认证

- 实现了FromRequest trait的类型可以从请求中被提取出来
- 例如:`Json<T>`
- 这种类型在Rocket中叫request guard，在Actix Web中叫extractor
- 那我们怎么利用这个 trait 实现身份认证呢?
  - 效仿Rocket，我们先定义两个struct: User和Admin
  - 为它们实现 FromRequest trait，从请求的cookies中提取登录时设置的ACCEss_TOKEN这个cookie
  - 然后像上一节一样做验证就可以了
  - 这样比使用中间件的方式更灵活，可以用于更缳杂的情况

## 10

设置数据库（建表)，查看评论，新增评论

建表

1. 进入数据库：`sudo -u postgres psql`
2. `\l`查看所有的数据库，`\c`选择使用数据库，`\d`查看所有表：我们输入`\c blog`，选择blog数据库
3. 输入下面的指令建表

```sql
CREATE TABLE IF NOT EXISTS comments(
   id SERIAL PRIMARY KEY,
   user_id INT4 NOT NULL,
   content varchar(1024) NOT NULL,
   date DATE DEFAULT CURRENT_DATE,
    article INT4 NOT NULL
);
```

## 11

查看，新增评论。路由设计：

- 查看评论：`GET /comment/{id}`(查看对应文章的所有评论)(包括评论的内容、日期和发表评论的用户的信息)
- 新增评论：`POST /comment (content, article)(USER)`

## 12

- 删除评论
  - 路由设计：DELETE /comment/{commint_id}
  - 用户只能删除自己写的评论，管理员可以随意删除评论
  - 我们还要修改上节写的获取评论的API，让它把评论ID返回给客户端

- 客户端（主页，404）
  - 一些概念
    - SPA (Single-page application，单页应用)
      - 如果你用过Outlook邮箱的网页版或者Github，你就会发现当我们点击网页上一些按钮或者链接的时候，并不需要让网页重新加载
      - SPA是只加载一个单独网页的web应用实现，当需要显示不同的内容时，它通过JavaScript API(例如XMLHttpRequest和Fetch)更新主体内容
      - 这使得用户在访问网站时无需加载新的页面，可以获得性能提升和动态的体验，但会相应地丧失诸如SEO(搜索引擎优化)的优势，同时需要更多精力维护状悉、实现导航以及做一些有意义的性能监控(MDN)
      - Yew提供了yew-router这个crate 用于为SPA提供路由的功能
      - yew-router 会根据浏览器的URL地址的变化而展示不同的页面，而不需要重新加载页面
    - Hooks(钩子)：
      - Hooks是用来存储数据让你执行一些操作（(产生一些副作用)( ? side-effects)的函数
      - 注意:
        - Hooks只能在函数组件(function components)的顶层使用，不能在普通的Rust函数里用
        - 其他的规则可以到yew.rs上有关hooks的部分中查看，错误地使用hooks会导致编译时错误或运行时的paic (F12)
    - Context上下文
      - 我们都知道HTML元素有properties(属性)，当然，我们自定义的组件也可以有属性，组件可以获取到组件的数据
      - 但是如果我们有一个全局的数据，那我们总不能给每个组件都传递一个属性吧，这样非常的麻烦
      - Context就相当于一个全局的属性，可以把数据传递给下面的每个组件
      - 在函数组件里，我们可以通过use_context::<T>()这个hook获取到Context中的某一个数据
    - Callbacks(回调函数)
      - 在Yew，数据流是从上往下的(父组件向子组件传递数据)，而通过使用callbacks，可以让子组件向上与父组件进行通信
      - 我们可以把一个函数(闭包)通过属性传递给一个子组件，然后子组件可以调用这个函数(闭包)，从而进行一些操作
      - HTML button元素有onclick这个属性，定义了按钮被点击时所发生的行为，要通过WebAssembly (Yew)处理这个点击事件，可以把一个callback传给onclick这个属性。这个callback的参数是被点击的HTML按钮本身，然后我们就可以在这个闭包里干一些事情了

## 13

在客户端实现预览文章的功能

一些概念：

- State
  - State类似变量，但是当state改变的时候，组件就会重新渲染，更新HTML 里面的值
  - 使用use_state(|l ...)创建一个state，然后通过state 上面的set(...)方法改变state里面的值
- use_effect use_effect_with deps是Hook
  - use effect 会在组件每次被渲染之后执行(接收一个闭包)，而use_effect_with_deps有第二个参数(deps)，可以监测它(们)的变化，在这些deps发生变化时执行闭包，把deps设为()，可以让里面的闭包只在组件挂载成功时执行

向后端API发送请求：

- 使用到的crate:
  - gloo:提供了在web-sys的原始JS API绑定之上的API封装，让我们能够更方便地编写WASM程序，包含网络请求、控制台( console)等功能
  - serde 与serde json:序列化、反序列化
  - wasm bindgen futures:能够将 Rust 的Future转为S的Promise。它提供了一个spawn local函数，可以在当前线程执行 Rust Future (WASM暂时还不支持多线程)

- 我们需要使用use_effect_with_deps这个hook，让我们能够在组件挂载成功时发送请求，因为只有在这个时候发送请求，我们对state的更改才会引起组件的重新渲染

## 14

- 修改fetch::fetch函数，让他能够返回服务端返回的错误消息

- 另外，我把articles.rs改成了article_preview.rs了，相应的引用也要改一下

实现查看文章的功能

- 我们的博客应用应该支持使用Markdown编写文章
- 那我们就得把后端返回的Markdown字符串转为HTML，然后显示在页面上
- 用什么库实现这个需求呢?
  - pulldown-cmark
  - 这是一个CommonMark 解析器，可以把CommonMark转为HTML字符串，具有快速、安全、规范等特点

> 什么是CommonMark，和Markdown有什么区别?
>
> CommonMark 对 Markdown的语法进行了规范，确保使用不同的parser渲染出来的效果是相同的
>
> 用Markdown的语法写就可以了

注意：如果添加或者更新一些依赖的版本时Trunk出现了wasm-bindgen(用于让RuSt Javan木不元配的问题，那你就需调用S，或者在JS中调用Rust)版本不匹配的问题，那你就需要去更新wasm-bindgen-cli，使用`cargo install wasm-bindaen-cli`来更新

- 我在把代码上传 Github的时候会把数据建表语句一起传上去，有需要的小伙伴可以自行查看
- 你得替换掉服务端代码中的CLIENT_ID和CLIENT_SECRET这两个常量和test.html中的client id

## 15

实现用户登录功能

- 服务端
  - 我们得加上获取用户信息的API
  - 路由设计: `GET /user/info`
  - 这个handler需要用户权限，会在我们的数据库中寻找对应的用户，把信息返回给用户

- 客户端
  - 做一个Github登录的功能(OAuth)(`/user/login和/user/login/oauth?code=xxx`)
  - 实现权限控制(把用户信息存在Context里，方便我们进行读取和更新)

这一章节需要到GitHub修改客户端返回的url，否则页面无法跳转回来（之前是跳转到test.html）

## 16

- 搜索文章
  - 在首页的卡片里加上一个输入框，当用户输入的时候就会进行搜索并展示结果
  - wasm-bindgen
    - 提供了在Rust 与JavaScript 之间通信的功能和JavaScript和Rust类型之间的桥梁
    - 它允许JavaScript使用字符串调用Rust API，或调用Rust函数来捕获JavaScript异常(互相调用)
  - JsCast
    - JS是动态类型的，现在它们在Rust 里有了静态的类型，这个trait 提供了把这些类型互相转换的功能，我们待会会用到

- 新增文章
  - /article/new
  - 编辑文章并新增文章
  - 支持实时预览的Markdown编辑器
  - 需要用户是管理员才展示新增文章的入口(按钮)
  - 即使普通用户通过/article/new这个路径进入了新增文章的页面，在尝试提交新文章的时候也会收到服务端返回的错误消息

## 17

修改和删除文章

## 18

> 如果trunk出现问题，可以尝试安装这个：`cargo install wasm-bindgen-cli`

## 19

删除评论

## 20

优化

- 主要是主页搜索的防抖
  - 以往，我们是在用户输入时就发送请求，进行搜索，这样会给服务端发送很多无效的请求（(用户还没输入完毕的时候仍然进行搜索)
  - 我们想要减少请求的数量，于是我们使用防抖:当用户还在输入时，不进行搜索，当用户停下来一定的时间后，才开始搜索- yew-hooks
- 其他小优化：CDN等

## 21

- 部署
  - Trunk内置的服务器只能用于开发，如果我们想要把我们的应用公开给其他用户访问，我们就需要一个HTTP server
  - 我们选用的HTTP server是Caddy v2
  - 这是一个使用Go语言编写的Web Server，具有配置简单、功能强大、可扩展等优点

```bash
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy
```

- 客户端
  - 安装完成后，我们需要配置Caddy，以便Caddy可以serve我们的网站
  - 我们使用Caddyfile的方式进行配置，可以到Caddy v2的文档了解更多配置项

![](https://moe.photo/images/2024/01/08/image-20240108173513334.png)

- 客户端

  - 我们还要配置一下Trunk，让它在release模式下编译我们的代码(我们要发布我们的应用)，减小.wasm文件的体积，加快运行速度
  - 可以顺便也用release来 build我们的server
  - 我们要把我们编译出来的可执行文件复制到一个单独的目录里，以与我们的项目文件分开

  - 服务端的部署就要简单得多，在把可执行文件复制到单独的目录之后，我们只需要设置好DATABASE_URL这个环境变量,就可以把我们的服务端跑起来了


设置环境变量，然后启动后端：`caddy run`，前端：`./serve`























