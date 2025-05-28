//! Catch literal strings in view! macros

#[macro_export]
macro_rules! view {
    ($($arg:tt)*) => {
        println!("foo");
    }
}

mod ids {
    #[allow(dead_code)]
    pub enum Ids {
        MyIdentifier,
    }
}

fn main() {
    view! {
        <div id="my-identifier">Hello, world!</div>
    }

    // Use the Ids enum instead
    #[allow(unused_imports)]
    use ids::Ids;

    view! {
        <div id=Ids::MyIdentifier>Hello, world!</div>
    }
}
