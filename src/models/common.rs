use cdrs::{
    types::{
        from_cdrs::FromCDRSByName,
        prelude::*,
        rows::Row,
    }
};
use std::result::Result;
use crate::config::db::Connection;
use cdrs::query::*;
use crate::config::CASSANDRA_DB_NAME;
use actix_web::{web, Error as AWError};
use failure::Error;
use futures::{Future, TryFutureExt};
use cdrs::types::{ByIndex};

#[derive(Clone, Debug, TryFromRow)]
pub struct CountStruct {
    pub count: i64
}
pub struct DbQuery;

impl DbQuery {
    pub async fn exec_db_query(conn: &Connection, prepared_query: PreparedQuery, query_value: QueryValues) -> Result<Vec<Row>, String> {
        let with_tracing = true;
        let with_warnings = true;
        match conn.exec_with_values_tw(&prepared_query, query_value, with_tracing, with_warnings) {
            Ok(result) => {
                let rows = result.get_body().expect("get body").into_rows().expect("into rows");
                return Ok(rows)
            },
            Err(ref err) => panic!("can't exec query {:?}", err),
        }
    }

    pub async fn get_count(conn: &Connection, table_name: &str, where_string: &str, where_value: QueryValues) -> Result<i64, String> {
        let prepared_query = conn.prepare(format!("SELECT count(*) FROM {}.{} WHERE {}", CASSANDRA_DB_NAME, table_name, where_string)).unwrap();
        let rows = Self::exec_db_query(&conn, prepared_query, where_value).await.expect("exec db query");
        if rows.len() > 0 {
            let row = rows.get(0).unwrap().to_owned();
            // let r1 = row.clone().by_name::<i64>("count").unwrap().unwrap();
            let count_struct: CountStruct = CountStruct::try_from_row(row).expect("into RowStruct");
            return Ok(count_struct.count)
        }
        Err(format!("record not exist"))
    }

    pub async fn get_rows(conn: &Connection, table_name: &str, select_string: &str, where_string: &str, where_value: QueryValues) -> Result<Vec<Row>, String> {
        let query = format!("SELECT {} FROM {}.{} WHERE {}", select_string, CASSANDRA_DB_NAME,
                            table_name, where_string);
        // println!("get_rows query = {} ", query);
        let prepared_query = conn.prepare(query).unwrap();
        let rows = Self::exec_db_query(&conn, prepared_query, where_value).await.expect("exec db query");
        Ok(rows)

    }
    pub async fn get_row(conn: &Connection, table_name: &str, select_string: &str, where_string: &str, where_value: QueryValues) -> Result<Row, String> {
        let query = format!("SELECT {} FROM {}.{} WHERE {}", select_string, CASSANDRA_DB_NAME, table_name, where_string);
        println!("get_row query ===== {} ", query);
        let prepared_query = conn.prepare(query).unwrap();
        let rows = Self::exec_db_query(&conn, prepared_query, where_value).await.expect("exec db query");
        if rows.len() > 0 {
            let row = rows.get(0).unwrap().to_owned();
            return Ok(row)
        }
        Err(format!("record not exist"))
    }
    pub async fn insert(conn: &Connection, table_name: &str, fields_str: &str, values_str: &str, values: QueryValues) -> Result<bool, String> {
        let query = format!("INSERT INTO {}.{} ({}) VALUES ({}) IF NOT EXISTS", CASSANDRA_DB_NAME, table_name, fields_str, values_str);
         println!("insert query ===== {} ", query);
        let prepared_query = conn.prepare(query).unwrap();
        let with_tracing = true;
        let with_warnings = true;
        match conn.exec_with_values_tw(&prepared_query, values, with_tracing, with_warnings) {
            Ok(result) => {
                let rows = result.get_body().expect("get body").into_rows().expect("into rows");
                // println!("insert result rows === {:?}", rows);
                let row = rows.get(0).unwrap().to_owned();
                // println!("insert result row === {:?}", row);
                match row.by_index::<bool>(0) {
                    Ok(is_inserted) => {
                        return Ok(is_inserted.unwrap())
                    },
                    Err(ref err) => panic!("can't exec query {:?}", err),
                }
            },
            Err(ref err) => panic!("can't exec query {:?}", err),
        }
        Err(format!("can't insert"))
    }
    pub async fn update(conn: &Connection, table_name: &str, update_fields_str: &str, where_string: &str, values: QueryValues) -> Result<bool, String> {
        let query = format!("UPDATE {}.{} SET {} WHERE {}", CASSANDRA_DB_NAME, table_name, update_fields_str, where_string);
        println!("update query ===== {} ", query);
        let prepared_query = conn.prepare(query).unwrap();
        let with_tracing = true;
        let with_warnings = true;
        match conn.exec_with_values_tw(&prepared_query, values, with_tracing, with_warnings) {
            Ok(result) => {
                let rows = result.get_body().expect("get body").into_rows().expect("into rows");
                // println!("insert result rows === {:?}", rows);
                let row = rows.get(0).unwrap().to_owned();
                 println!("update result row === {:?}", row);
                match row.by_index::<bool>(0) {
                    Ok(is_updated) => {
                        return Ok(is_updated.unwrap())
                    },
                    Err(ref err) => panic!("can't exec query {:?}", err),
                }
            },
            Err(ref err) => panic!("can't exec query {:?}", err),
        }
        Err(format!("can't insert"))
    }
    pub async fn delete(conn: &Connection, table_name: &str, where_string: &str, values: QueryValues) -> Result<bool, String> {
        let query = format!("DELETE FROM {}.{} WHERE {}", CASSANDRA_DB_NAME, table_name, where_string);
        println!("delete query ===== {} ", query);
        let prepared_query = conn.prepare(query).unwrap();
        let with_tracing = true;
        let with_warnings = true;
        match conn.exec_with_values_tw(&prepared_query, values, with_tracing, with_warnings) {
            Ok(result) => {
                let rows = result.get_body().expect("get body").into_rows().expect("into rows");
                // println!("insert result rows === {:?}", rows);
                let row = rows.get(0).unwrap().to_owned();
                 println!("delete result row === {:?}", row);
                match row.by_index::<bool>(0) {
                    Ok(is_deleted) => {
                        return Ok(is_deleted.unwrap())
                    },
                    Err(ref err) => panic!("can't exec query {:?}", err),
                }
            },
            Err(ref err) => panic!("can't exec query {:?}", err),
        }
        Err(format!("can't insert"))
    }
}


