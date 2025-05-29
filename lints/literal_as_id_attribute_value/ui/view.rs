//! Catch literal strings in view! macros

#[macro_export]
macro_rules! view {
    ($($arg:tt)*) => {
        println!("foo");
    };
}

fn main() {
    view! {
        <div id="my-identifier">Hello</div>
    }
    // Use the Ids enum instead
    view! {
        <div id=Ids::MyIdentifier>Hello</div>
    }

    view! {
        <div id="another-identifier">Hello</div>
    }

    // attr:id syntax
    view! {
        <div attr:id="my-identifier">Hello</div>
    }
    view! {
        <div attr:id=Ids::MyIdentifier>Hello</div>
    }
}
