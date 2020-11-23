use sqlite::{
    Connection,
    open,
    Error,
    State,
    Value
};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex}
};
use crate::common::common::KeyValue;
pub struct DataBase {
    connect: Connection,
    path:   String,
}

thread_local! {
    pub static  DATABASE: RefCell<DataBase> = RefCell::new( DataBase::new("test.db"))
}

impl DataBase {
    pub fn new(path: &str) -> DataBase {
        let  connection = open(path).unwrap();

        // match connection {
        //     Err(_) => None,
        //     Ok(val) => Some(DataBase {
        //         connect: val,
        //         path: path.to_string()
        //     })
        // }
        DataBase {
                    connect: connection,
                    path: path.to_string()
        }
    }

    pub fn create_table(&mut self, table_name: &str, params: Vec<KeyValue>) -> Result<i32, Error> {
        let mut sql: String = String::new();

        if table_name.to_string().len() == 0 || params.len() == 0 {
            return Ok(-1)
        }
        sql.push_str("CREATE TABLE IF NOT EXISTS ");
        sql.push_str(table_name);
        sql.push_str(" ( ");
        for param in params {
            sql.push_str(&param.key);
            sql.push_str("  ");
            sql.push_str(&param.value);
            sql.push_str(",\n");
        }
        sql.pop();
        sql.pop();
        sql.push(')');
        self.connect.execute(&sql)?;
        Ok(0)
    }

    pub fn check_table(&mut self, table_name: &str) -> Result<bool, sqlite::Error> {
        let mut sql: String = String::new();
        let mut flag: bool = false;
        sql.push_str("SELECT COUNT(*) FROM sqlite_master ");
        sql.push_str(r#"WHERE type = "table" "#);
        sql.push_str(r#" and name = ?"#);
        {
            let mut statement = self.connect.prepare(&sql)?;

            statement.bind(1, table_name)?;
            statement.next()?;

            let result = statement.read::<i64>(0)?;

            if result == 0 {
                flag = false;
            } else {
                flag = true;
            }
        }
        
        Ok(flag)
    }

    pub fn delete_table(&mut self, table_name: &str) -> Result<bool, Error> {
        let mut sql: String = String::from("DROP TABLE ");

        sql.push('\"');
        sql.push_str(table_name);
        sql.push('\"');
        self.connect.execute(sql)?;
        Ok(true)
    }
    
    // fn alter_table() -> Result<u8, sqlite3::SqliteError> {
    //     1
    // }

    fn insert_one_data(&mut self, table_name: &str, params: Vec<KeyValue>) -> Result<bool, Error> {
        let mut sql: String = String::from(r#"INSERT INTO ? VALUES ( "#);

        for _i in 0..params.len() {
            sql.push_str("?,");
        }
        sql.pop();
        sql.push(')');
        sql.push(' ');
        sql.push_str("VALUES ( ");
        for _i in 0..params.len() {
            sql.push_str("?,");
        }
        sql.pop();
        sql.push(')');
        println!("{}", sql);
        {
            let mut statement = self.connect.prepare(&sql)?;
            let len  = params.len();

            for i in 0..len {
                let key: &str = &params[i].key;
                let value: &str = &params[i].value;
                statement.bind(i, key)?;
                statement.bind(i + len, value)?;
            }
            statement.next()?;
        }

        Ok(true)
    }
    // fn insert_many_data() -> Result<u8, sqlite3::SqliteError> {
    //     1
    // }
    fn select_data(&mut self, table_name: &str, params: Vec<KeyValue>) -> Result<Vec<Vec<String>>, Error> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut sql: String = String::from(r#"SELECT FROM ? WHERE "#);
        
        {
            let len = params.len();
            let and = String::from(" and ");
            let mut temp = String::new();

            for kv in &params {

                temp.push_str(&kv.key);
                temp.push_str(" = ");
                temp.push_str(&kv.value);
                temp.push_str(&and);
            }
            for _i in 0..and.len() {
                temp.pop();
            }

            sql.push_str(&temp);

        }
        {
            let mut statement = self.connect.prepare(&sql)?.cursor();

            statement.bind(&[Value::String(table_name.to_string())])?;
            while let Some(row) = statement.next()? {
                let mut temp: Vec<String> = Vec::new();
                let len = row.len();

                if len == 0 {
                    continue;
                }
                for i in 0..len {
                    temp.push(statement.read::<String>(0)?)
                } 
            }
        }
        println!("{}", sql);

        Ok(result)
    }
    // fn delete_data() -> Result<u8, sqlite3::SqliteError> {
    //     1
    // }
    // fn delete_add() -> Result<u8, sqlite3::SqliteError> {
    //     1
    // }
}