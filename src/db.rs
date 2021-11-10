use std::path::PathBuf;

use log::info;
use rusqlite::{params, Connection, Error as DBError};
use chrono::DateTime;

use crate::config::AppConfig;
use crate::stories::Story;

const SCHEMA_SQL: &str = "
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

const INSERT_STORY_SQL: &str = "
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

const HIGHEST_SCORING_STORIES_SQL: &str = "
    SELECT
        permalink,
        subreddit,
        title,
        score,
        created_utc,
        author,
        num_comments,
        url
    FROM STORIES
    ORDER BY SCORE + 20 * NUM_COMMENTS DESC
    LIMIT 25;
";

pub struct DB {
    connection: Connection,
}

impl DB {
    pub fn new() -> Result<Self, DBError> {
        let db_path = AppConfig::get_db_path();
        if !db_path.exists() {
            DB::make_new_db(&db_path)
        } else {
            DB::open_db(&db_path)
        }
    }

    fn make_new_db(path: &PathBuf) -> Result<Self, DBError> {
        info!("Could not find stories database, creating a new one");
        let db = DB::open_db(&path)?;
        db.connection.execute(SCHEMA_SQL, [])?;
        Ok(db)
    }

    fn open_db(path: &PathBuf) -> Result<Self, DBError> {
        info!("Opening stories database");
        let connection = Connection::open(&path)?;
        Ok(Self { connection })
    }

    fn save_story(&mut self, story: &Story) -> Result<usize, DBError> {
        self.connection.execute(
            INSERT_STORY_SQL,
            params![
                story.permalink,
                story.subreddit,
                story.title,
                story.score,
                story.get_created_utc_iso8601(),
                story.author,
                story.num_comments,
                story.url
            ],
        )
    }

    pub fn save_stories(&mut self, stories: &[Story]) -> Result<(), DBError> {
        info!("Writing stories to database");
        for s in stories {
            self.save_story(s)?;
        }

        Ok(())
    }

    pub fn get_highest_scoring_stories(&mut self) -> Result<Vec<Story>, DBError> {
        info!("Getting highest scoring stories from the database");

        // TODO: really living on the edge in this function.
        let mut stmt = self.connection.prepare(HIGHEST_SCORING_STORIES_SQL)?;
        let story_iter = stmt.query_map([], |row| {
            let created_utc: String = row.get(4)?;
            let created_utc = DateTime::parse_from_rfc3339(&created_utc).unwrap();
            let created_utc = created_utc.timestamp();
            Ok(Story {
                permalink: row.get(0)?,
                subreddit: row.get(1)?,
                title: row.get(2)?,
                score: row.get(3)?,
                created_utc: created_utc as f64,
                author: row.get(5)?,
                num_comments: row.get(6)?,
                url: row.get(7)?,
            })
        })?;

        Ok(story_iter.map(Result::unwrap).collect())
    }
}
