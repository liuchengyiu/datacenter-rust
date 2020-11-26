use sqlite::{
    Connection,
    open,
    Error,
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
    pub static  DATABASE: Arc<Mutex<RefCell<DataBase>>> = Arc::new(Mutex::new(RefCell::new( DataBase::new("test.db"))))
}

impl DataBase {
    pub fn new(path: &str) -> DataBase {
        let  connection = open(path).unwrap();

        DataBase {
                    connect: connection,
                    path: path.to_string()
        }
    }

    pub fn init_base_table(&mut self) {
        let table_names:Vec<&str> = Vec::from(["model", "device"]);
        let kvs: Vec<Vec<KeyValue>> = Vec::from([
            Vec::from([
                KeyValue {key: "name".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "params".to_string(), value:"TEXT".to_string()}
            ]),
            Vec::from([
                KeyValue {key: "model".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "port".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "addr".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "desc".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "guid".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "appname".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "manuID".to_string(), value:"TEXT".to_string()},
                KeyValue {key: "isreport".to_string(), value:"TEXT".to_string()},
            ])
        ]);
        let len = table_names.len();

        for i in 0..len {
            self.create_table(table_names[i], &kvs[i]);
        }
        println!("init database tables end");
    }

    pub fn create_table(&mut self, table_name: &str, params: &Vec<KeyValue>) -> Result<bool, Error> {
        let mut sql: String = String::new();

        if table_name.to_string().len() == 0 || params.len() == 0 {
            return Ok(false)
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
        Ok(true)
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
    
    pub fn exists_data(&mut self, table_name: &str, params: &Vec<KeyValue>) -> Result<bool, Error> {
        let mut sql: String = String::new();
        let mut flag: bool = false;

        sql.push_str("SELECT COUNT(*) FROM ");
        sql.push_str(table_name);
        sql.push_str(" WHERE ");
        {
            let and = String::from(" and ");
            let mut temp = String::new();

            for kv in params {

                temp.push_str(&kv.key);
                temp.push_str(" = ");
                temp.push('\"');
                temp.push_str(&kv.value);
                temp.push('\"');
                temp.push_str(&and);
            }
            for _i in 0..and.len() {
                temp.pop();
            }

            sql.push_str(&temp);
        }
        {
            println!("{}", sql);
            let mut statement = self.connect.prepare(&sql)?;

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

    // fn alter_table() -> Result<u8, sqlite3::SqliteError> {
    //     1
    // }

    pub fn insert_one_data(&mut self, table_name: &str, params: &Vec<KeyValue>) -> Result<bool, Error> {
        let mut sql: String = String::from(r#"INSERT INTO "#);
        sql.push_str(table_name);
        sql.push_str(" (");

        for i in 0..params.len() {
            sql.push_str(&params[i].key);
            sql.push(',');
        }
        sql.pop();
        sql.push(')');
        sql.push('\n');
        sql.push_str(r#"VALUES ("#);
        for i in 0..params.len() {
            sql.push_str("\"");
            sql.push_str(&params[i].value);
            sql.push_str("\"");
            sql.push(',');
        }
        sql.pop();
        sql.push(')');
        println!("{}", sql);
        {
            self.connect.execute(sql)?;
        }

        Ok(true)
    }
    
    pub fn insert_many_data(&mut self, table_name: &str, params: &Vec<Vec<KeyValue>>) -> Result<bool, Error> {
        for kvs in params {
            self.insert_one_data(table_name, kvs)?;
        }

        Ok(true)
    }
    
    pub fn select_data(&mut self, table_name: &str, params: &Vec<KeyValue>) -> Result<Vec<Vec<String>>, Error> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut sql: String = String::from(r#"SELECT * FROM "# );
        sql.push_str(table_name);
        sql.push_str(r#" WHERE "#);
        
        {
            let and = String::from(" and ");
            let mut temp = String::new();

            for kv in params {

                temp.push_str(&kv.key);
                temp.push_str(" = ");
                temp.push('\"');
                temp.push_str(&kv.value);
                temp.push('\"');
                temp.push_str(&and);
            }
            for _i in 0..and.len() {
                temp.pop();
            }

            sql.push_str(&temp);
        }
        println!("{}", sql);
        {
            let mut statement = self.connect.prepare(&sql)?.cursor();
            while let Some(row) = statement.next()? {
                let mut temp: Vec<String> = Vec::new();
                let len = row.len();

                if len == 0 {
                    continue;
                }
                for i in 0..len {
                    match row[i].as_string() {
                        None => temp.push(String::new()),
                        Some(val) => {
                            temp.push(val.to_string());
                        }
                    }
                }
                result.push(temp.clone());
            }
        }
        Ok(result)
    }

    pub fn select_all(&mut self, table_name: &str) -> Result<Vec<Vec<String>>, Error> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut sql: String = String::from(r#"SELECT * FROM "# );

        sql.push_str(table_name);
        {
            let mut statement = self.connect.prepare(&sql)?.cursor();
            while let Some(row) = statement.next()? {
                let mut temp: Vec<String> = Vec::new();
                let len = row.len();

                if len == 0 {
                    continue;
                }
                for i in 0..len {
                    match row[i].as_string() {
                        None => temp.push(String::new()),
                        Some(val) => {
                            temp.push(val.to_string());
                        }
                    }
                }
                result.push(temp.clone());
            }
        }
        Ok(result)
    }

    pub fn select_column_all(&mut self, table_name: &str, key: &str) -> Result<Vec<Vec<String>>, Error> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut sql: String = String::from(r#"SELECT "# );
        sql.push_str(key);
        sql.push_str(" FROM ");
        sql.push_str(table_name);
        {
            let mut statement = self.connect.prepare(&sql)?.cursor();
            while let Some(row) = statement.next()? {
                let mut temp: Vec<String> = Vec::new();
                let len = row.len();

                if len == 0 {
                    continue;
                }
                for i in 0..len {
                    match row[i].as_string() {
                        None => temp.push(String::new()),
                        Some(val) => {
                            temp.push(val.to_string());
                        }
                    }
                }
                result.push(temp.clone());
            }
        }
        Ok(result)
    } 

    pub fn select_by_colunm(&mut self, table_name: &str, keys: Vec<String>, params: &Vec<KeyValue>) -> Result<Vec<Vec<String>>, Error> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut sql: String = String::from(r#"SELECT "# );
        for k in keys {
            sql.push_str(&k);
            sql.push_str(", ");
        }
        sql.pop();
        sql.pop();
        sql.push_str(" FROM ");
        sql.push_str(table_name);
        sql.push_str(r#" WHERE "#);
        
        {
            let and = String::from(" and ");
            let mut temp = String::new();

            for kv in params {

                temp.push_str(&kv.key);
                temp.push_str(" = ");
                temp.push('\"');
                temp.push_str(&kv.value);
                temp.push('\"');
                temp.push_str(&and);
            }
            for _i in 0..and.len() {
                temp.pop();
            }

            sql.push_str(&temp);
        }
        println!("{}", sql);
        {
            let mut statement = self.connect.prepare(&sql)?.cursor();
            while let Some(row) = statement.next()? {
                let mut temp: Vec<String> = Vec::new();
                let len = row.len();

                if len == 0 {
                    continue;
                }
                for i in 0..len {
                    match row[i].as_string() {
                        None => temp.push(String::new()),
                        Some(val) => {
                            temp.push(val.to_string());
                        }
                    }
                }
                result.push(temp.clone());
            }
        }
        Ok(result)
    }

    pub fn update_data(&mut self, table_name: &str, conditions: &Vec<KeyValue> ,params: &Vec<KeyValue>) -> Result<bool, Error> {
        let mut sql: String = String::from("UPDATE ");
        
        sql.push_str(table_name);
        sql.push(' ');
        sql.push_str("SET ");
        {
            let and = String::from(" and ");
            let mut temp = String::new();

            for kv in params {

                temp.push_str(&kv.key);
                temp.push_str(" = ");
                temp.push('\"');
                temp.push_str(&kv.value);
                temp.push('\"');
                temp.push_str(&and);
            }
            for _i in 0..and.len() {
                temp.pop();
            }

            sql.push_str(&temp);
        }
        sql.push_str(" WHERE ");
        {
            let and = String::from(" and ");
            let mut temp = String::new();

            for kv in conditions {

                temp.push_str(&kv.key);
                temp.push_str(" = ");
                temp.push('\"');
                temp.push_str(&kv.value);
                temp.push('\"');
                temp.push_str(&and);
            }
            for _i in 0..and.len() {
                temp.pop();
            }

            sql.push_str(&temp);
        }
        println!("{}", sql);
        self.connect.execute(sql)?;

        Ok(true)
    }

    pub fn delete_data(&mut self, table_name: &str, params: &Vec<KeyValue>) -> Result<bool, Error> {
        let mut sql: String = String::from("DELETE FROM ");
        let and = String::from(" and ");

        sql.push_str(table_name);
        sql.push_str(" WHERE ");
        for kv in params {
            sql.push_str(&kv.key);
            sql.push_str(" = ");
            sql.push('\"');
            sql.push_str(&kv.value);
            sql.push('\"');
            sql.push_str(&and);
        }
        for _i in 0..and.len() {
            sql.pop();
        }
        self.connect.execute(sql)?;

        Ok(true)
    }

    pub fn delete_all(&mut self, table_name: &str) -> Result<bool, Error> {
        let mut sql: String = String::from("DELETE FROM ");

        sql.push_str(table_name);
        self.connect.execute(sql)?;
        Ok(true)
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}