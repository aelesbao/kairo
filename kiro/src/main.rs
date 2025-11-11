use kiro::{KiroResult, handlers_for_scheme};

fn main() -> KiroResult<()> {
    for app in handlers_for_scheme("http")? {
        println!("  {:#?}", app);
    }

    Ok(())
}
