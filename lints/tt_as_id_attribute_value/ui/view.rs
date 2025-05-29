//! Catch literal strings in view! macros

#[macro_export]
macro_rules! view {
    ($($arg:tt)*) => {
        println!("View macro called with: {}", stringify!($($arg)*));
    };
}

mod ids {
    #[allow(dead_code)]
    pub enum Ids {
        MyIdentifier,
    }
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
    #[allow(unused_imports)]
    use ids::Ids;

    view! {
        <div id=Ids::MyIdentifier>Hello, world!</div>
    }
}
