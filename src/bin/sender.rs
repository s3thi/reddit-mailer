use std::error::Error;

use librm::db::DB;

fn main() -> Result<(), Box<dyn Error>> {
    let mut db = DB::new()?;
    let stories = db.get_highest_scoring_stories()?;

    let stories_list = stories
        .iter()
        .map(|s| {
            format!(
                r#"<li><a href="{}">{}</a> (<a href="https://www.reddit.com{}">comments</a>)</li>"#,
                s.url, s.title, s.permalink
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let stories_markup = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <h1>Recent Top Reddit Stories</h1>
            <ol>
                {}
            </ol>
        </body>
        </html>
    "#,
        stories_list
    );

    Ok(())
}
