use clap::App;
use dirs;
use rusqlite::{params, Connection, Result};
use std::process::Command;

const DB_LOCATION: &str =
    "Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear/Application Data/database.sqlite";

#[derive(Debug)]
enum BearCommand {
    Random,
}

#[derive(Debug)]
struct Note {
    title: String,
    subtitle: String,
    text: String,
    unique_id: String,
}

fn main() -> Result<()> {
    let matches = App::new("bear-cli")
        .version("0.1")
        .author("Jeremy Keeler <jerkeeler@gmail.com>")
        .args_from_usage("<COMMAND> 'The command that you want to run. One of: random'")
        .get_matches();

    let command = match matches.value_of("COMMAND").unwrap() {
        "random" => BearCommand::Random,
        _ => {
            println!("Invalid command provided! Please choose one of: random");
            return Ok(());
        }
    };

    let connection = connect_db();

    let res = match command {
        BearCommand::Random => random_note(&connection),
    };

    connection.close().expect("Failed to close db connection!");
    res.expect("Failed to run bear command!");
    Ok(())
}

fn connect_db() -> Connection {
    let home_dir = dirs::home_dir();
    let mut home_dir = match home_dir {
        Some(dir) => dir,
        None => panic!("No home directory found! Cannot locate database!"),
    };
    home_dir.push(DB_LOCATION);

    let conn = Connection::open(home_dir.as_path());
    let conn = match conn {
        Ok(conn) => conn,
        _ => panic!("Invalid connection!"),
    };
    conn
}

fn random_note(conn: &Connection) -> Result<()> {
    println!("Choosing random note...");
    let mut stmt = conn.prepare(
        "SELECT ZTITLE,  ZSUBTITLE, ZTEXT, ZUNIQUEIDENTIFIER
            FROM ZSFNOTE
            WHERE ZARCHIVEDDATE IS NULL
                AND ZTRASHEDDATE IS NULL
            ORDER BY RANDOM()
            LIMIT 1
            ",
    )?;
    let mut res_iter = stmt.query_map(params![], |row| {
        Ok(Note {
            title: row.get(0)?,
            subtitle: row.get(1)?,
            text: row.get(2)?,
            unique_id: row.get(3)?,
        })
    })?;
    let res: Note = res_iter.next().unwrap()?;
    open_note(res);

    Ok(())
}

fn open_note(note: Note) {
    let url = format!(
        "bear://x-callback-url/open-note?id={}&new_window=yes&show_window=yes",
        note.unique_id
    );
    Command::new("open")
        .arg(url)
        .output()
        .expect("Failed to execute process!");
}
