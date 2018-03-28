// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
struct ErrorA;
struct ErrorB;

enum Error {
    A(ErrorA),
    B(ErrorB)
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    let a = do_a();
    let a = match a {
        Ok(res) => res,
        Err(err) => { panic!("Error!") }
    };
    let b = do_b();
    let b = match b {
        Ok(res) => res,
        Err(err) => { panic!("Error!") }
    };
    Ok((a, b))
}

fn main() { }
