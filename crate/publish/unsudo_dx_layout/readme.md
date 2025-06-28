```rs
    use ::unsudo_dx_layout as layout;

    pub fn HelloWorld() -> Element {
        rsx! {
            layout::Row {
                style: r#"
                    gap: 10px;
                    font-family: monospace;
                    font-size: 4em;
                    font-weight: normal;
                "#,
                layout::Row { "Hello" }
                layout::Row { "World" }
            }
        }
    }
```