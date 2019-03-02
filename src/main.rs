struct Foo {
    title: String,
}

type Result<T> = std::result::Result<T, &'static str>;

impl Foo {
    fn new(title: &str) -> Result<Foo> {
        Ok(Foo {
            title: title.to_string(),
        })
    }

    fn do_something(&self) -> Result<u32> {
        println!("do_something");
        Err("do_something failed")
    }

    fn do_something_else(&self) -> Result<u32> {
        println!("do_something_else");
        Ok(456)
    }
}

fn run() -> Result<()> {
    let foo = Foo::new("some-title")?;
    println!("title={:?}", foo.title);
    let value = foo.do_something()
                    .or_else(|_| foo.do_something_else())?;
    println!("value={:?}", value);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => println!("Success"),
        Err(e) => println!("Failure: {:?}", e),
    }
}
