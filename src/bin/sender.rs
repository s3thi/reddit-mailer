use std::error::Error;

use librm::db::DB;

fn main() -> Result<(), Box<dyn Error>> {
    let mut db = DB::new()?;
    let stories = db.get_highest_scoring_stories()?;
    for s in stories {
        println!("{}", s.title);
    }

    Ok(())
}
