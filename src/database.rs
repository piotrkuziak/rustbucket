use crate::task::Task;
use crate::task::Status;
use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open() -> Result<Self> {
        let conn = Connection::open("tasks.sqlite")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  status TEXT NOT NULL
                  )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn create_task(&self, task: Task) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (name, status) VALUES (?1, ?2)",
            (&task.name, &task.status.to_string()),
        )?;
        Ok(())
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks")?;

        let rows = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                status: match row.get::<usize, String>(2)?.as_str() {
                    "To do" => Status::ToDo,
                    "In progress" => Status::InProgress,
                    "Done" => Status::Done,
                    _ => Status::Unknown
                }
            })
        })?;

        // Collect the rows into a vector
        let result: Result<Vec<_>> = rows.collect();
        result
    }

    pub fn update_task(&self, task_id: usize, new_status: Status) -> Result<()> {
        let status = match new_status {
            Status::ToDo => "To do",
            Status::InProgress => "In progress",
            Status::Done => "Done",
            Status::Unknown => "Unknown",
        };

        self.conn.execute("UPDATE tasks SET status = ?1 WHERE id = ?2", (status.to_string(), task_id))?;

        Ok(())
    }

    pub fn delete_task(&self, task_id: usize) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1",[&task_id])?;
        Ok(())
    }

    pub fn delete_all_tasks(&self) -> Result<()> {
        self.conn.execute("DELETE FROM tasks",[])?;
        Ok(())
    }
}