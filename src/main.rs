use sqlite::State;

struct Folder {
  id: String,
  name: String,
}

struct Note {
  id: String,
  folder: String,
  name: String,
  text: String,
}

fn table_exists(table_name: String) -> bool {
  let connection = sqlite::open("./database.db").unwrap();

  let mut statement = connection
    .prepare(
      "
      select count(*) as count from sqlite_master
      where type = 'table' and name = ?
    ",
    )
    .unwrap();

  statement.bind(1, &*table_name).unwrap();

  let mut count = String::from("0");

  while let State::Row = statement.next().unwrap() {
    count = statement.read::<String>(0).unwrap_or(String::from("0"));
  }

  return count != "0";
}

fn create_notes_table() {
  let connection = sqlite::open("./database.db").unwrap();

  let statement = String::from(
    "
      CREATE TABLE notes (
       id TEXT NOT NULL,
       folderId TEXT NOT NULL,
       text BLOB,
       name TEXT NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(folderId) REFERENCES folders(id)
      )
    ",
  );

  connection.execute(statement).unwrap();
}

fn create_folders_table() {
  let connection = sqlite::open("./database.db").unwrap();

  let statement = String::from(
    "
      CREATE TABLE folders (
        id TEXT,
        name TEXT,
        PRIMARY KEY(id)
      )
    ",
  );

  connection.execute(statement).unwrap();
}

fn create_folder(folder: Folder) -> Folder {
  if !table_exists(String::from("folders")) {
    create_folders_table();
  }

  let connection = sqlite::open("./database.db").unwrap();
  let mut statement = connection
    .prepare(
      "
        insert into folders (
          id,
          name
        ) values (
          ?,
          ?
        )
      ",
    )
    .unwrap();

  statement.bind(1, &*folder.id).unwrap();
  statement.bind(2, &*folder.name).unwrap();

  statement.next().unwrap();

  println!("Creating folder");
  println!("  - id: {}", folder.id);
  println!("  - name: {}", folder.name);

  return folder;
}

fn create_note(note: Note) {
  if !table_exists(String::from("notes")) {
    create_notes_table();
  }

  let connection = sqlite::open("./database.db").unwrap();
  let mut statement = connection
    .prepare(
      "
        insert into notes (
          id,
          folderId,
          name,
          text
        ) values (
          ?,
          ?,
          ?,
          ?
        )
      ",
    )
    .unwrap();

  statement.bind(1, &*note.id).unwrap();
  statement.bind(2, &*note.folder).unwrap();
  statement.bind(3, &*note.name).unwrap();
  statement.bind(4, &*note.text).unwrap();

  statement.next().unwrap();

  println!("Creating note");
  println!("  - id: {}", note.id);
  println!("  - name: {}", note.name);
  println!("  - folder: {}", note.folder);
  println!("  - text: {}", note.text);
}

fn main() {
  let folder = Folder {
    id: String::from("1234"),
    name: String::from("Some Folder"),
  };

  let created_folder = create_folder(folder);

  let note = Note {
    id: String::from("1234"),
    name: String::from("Note 1"),
    folder: created_folder.id,
    text: String::from("This is a test note."),
  };

  create_note(note)
}
