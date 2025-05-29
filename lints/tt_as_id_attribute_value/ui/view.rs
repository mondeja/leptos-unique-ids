//! Catch literal strings in view! macros

#[macro_export]
macro_rules! view {
    ($($arg:tt)*) => {
        println!("View macro called with: {}", stringify!($($arg)*));
    };
}

fn main() {
    #[allow(unused_variables)]
    let foo = "my-identifier";

    view! {
        <div id=foo>Hello, world!</div>
    }

    view! {
        <div id={foo}>Hello, world!</div>
    }

    // Use the Ids enum instead

    view! {
        <div id=Ids::MyIdentifier>Hello, world!</div>
    }

    // this case is catched by `literal_as_id_attribute_value` lint,
    // so it should not trigger here
    view! {
        <div id="my-identifier">Hello, world!</div>
    }

    view! {
        <div id={
            let my_id = "my-identifier";
            my_id
        }>Hello, world!</div>
    }
}
