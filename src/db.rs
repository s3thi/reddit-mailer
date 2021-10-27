use crate::stories::Story;
use rusqlite::{params, Connection, Error as DBError};
use std::path::Path;

const SCHEMA_SQL: &'static str = "
    CREATE TABLE stories (
        permalink TEXT PRIMARY KEY,
        subreddit TEXT NOT NULL,
        title TEXT NOT NULL,
        score INTEGER NOT NULL,
        created_utc TEXT NOT NULL,
        author TEXT NOT NULL,
        num_comments INTEGER NOT NULL,
        url TEXT NOT NULL
    );
";

const INSERT_STORY_SQL: &'static str = "
    INSERT INTO stories (
        permalink,
        subreddit,
        title,
        score,
        created_utc,
        author,
        num_comments,
        url
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    ON CONFLICT(permalink) DO UPDATE SET
        score = excluded.score,
        num_comments = excluded.num_comments;
";

pub struct DB {
    connection: Connection,
}

impl DB {
    pub fn new() -> Result<Self, DBError> {
        // TODO: pick up DB path from a config file.
        if !Path::new("reddit-mailer.sqlite").exists() {
            DB::make_new_db()
        } else {
            DB::open_db()
        }
    }

    fn make_new_db() -> Result<Self, DBError> {
        let db = DB::open_db()?;
        db.connection.execute(SCHEMA_SQL, [])?;
        Ok(db)
    }

    fn open_db() -> Result<Self, DBError> {
        let connection = Connection::open("reddit-mailer.sqlite")?;
        Ok(Self { connection })
    }

    pub fn save_story(&mut self, story: &Story) -> Result<(), DBError> {
        let created_iso8601 = "xxx";
        self.connection.execute(
            INSERT_STORY_SQL,
            params![
                story.permalink,
                story.subreddit,
                story.title,
                story.score,
                created_iso8601,
                story.author,
                story.num_comments,
                story.url
            ],
        )?;
        Ok(())
    }
}