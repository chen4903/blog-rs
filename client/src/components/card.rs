use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    // 由String/&str自动转换而来。用AttrValue的原因：
    // Props里面的字段，他们在内部都被实现为引用计数的，用Rc包裹起来，如果用String，其clone成本较高
    // 而AttrValue的成本较低，yew推荐使用
    pub title: AttrValue, 
    pub children: Children,
}

/*
pub children: Children, 这个Children是啥，为什么要填这个?
Children 是 Yew 框架中用于表示组件子元素的特殊类型。在 Yew 中，组件的子元素可以是各种类型，包括原始 HTML 元素、其他组件、文本等。Children 类型提供了一种方便的方式来处理这些不同类型的子元素。

在你提供的代码中，pub children: Children 表示卡片组件的属性中包含一个名为 children 的字段，该字段的类型是 Children。通过使用 Children 类型，可以使卡片组件接受不同类型的子元素，并在渲染时适当处理它们。

在 Yew 中，可以将多个子元素传递给组件，而不需要特定的容器元素。例如：
```rust
<Card title=AttrValue::new("My Card")>
    <p>{"This is the content of the card."}</p>
    <button>{"Click me"}</button>
</Card>
```

在这个例子中，<p> 和 <button> 元素就是 Card 组件的子元素，它们会被作为 Children 类型的对象传递给 card 函数的 props.children。

使用 Children 类型的好处在于它提供了一种灵活的方式来处理不同类型的子元素，而无需为每种情况定义不同的属性。在组件的渲染函数中，可以使用 for props.children.iter() 迭代子元素，并根据需要进行处理。

 */

#[function_component(Card)] // 使用 #[function_component] 属性宏定义了一个 Yew 组件函数 card。
pub fn card(props: &Props) -> Html { // props: &Props 参数表示组件的属性，通过解引用获得属性值。
    html! { // 在函数体内使用 html! 宏构建了组件的虚拟 DOM 结构。
        <article class="card" style="margin: auto; margin-top: 5%; width: 80%;">
        // 标题部分使用 <h3> 元素，内容部分使用 {for props.children.iter()} 迭代子组件。
            <header>
                <h3>{&props.title}</h3>
            </header>
            <footer>
                {for props.children.iter()}
            </footer>
        </article>
    }
}