//use datetime::{LocalDate, LocalDateTime, convenience::Today};
//use chrono::{Local, DateTime, Offset, TimeZone};

pub mod tasks {
    use rsfbclient::{prelude::*, FbError, Row};
    // use rsfbclient_core::ParamsType;

    use crate::fb_base::fb_conn::*;
    use chrono::{NaiveDate, NaiveDateTime};
    
    pub fn insert_task() -> Result<(), FbError> {
        let mut conn = new_conn().unwrap();
        conn.execute(
            "
            INSERT INTO TAREFA
            (TITULO, DESCRICAO, DATA_CADASTRO, DATA_LIMITE, PRIORIDADE)
            VALUES
            (?, ?, ?, ?, ?)
            ",
            (
                "Rust with Firebird",
                "Practice some Rust programming with Firebird",
                "2023.10.07 14:00:00",
                "2023.10.08 22:00:00",
                1
            )
        )?;
        Ok(())
    }

    pub fn insert_task_with_params() -> Result<(), FbError> {
        let mut conn = new_conn().unwrap();

        #[derive(Clone, IntoParams)]
        struct InsertParams {
            titulo: String,
            descricao: String,
            data_cadastro: NaiveDateTime,
            data_limite: NaiveDateTime,
            prioridade: i8
        }
        let params = InsertParams {
            titulo: "Tarefa Rust 3".to_string(),
            descricao: "Descrição teste 3".to_string(),
            // data_cadastro: "2023-10-07 20:00:00".to_string(),
            // data_limite: "2023-10-08 22:00:00".to_string(),
            data_cadastro: NaiveDate::from_ymd_opt(2023, 10, 7).unwrap().and_hms_opt(14, 10, 11).unwrap(),
            data_limite: NaiveDate::from_ymd_opt(2023, 10, 8).unwrap().and_hms_opt(22, 00, 00).unwrap(),
            prioridade: 1
        };
        conn.execute(
            "
            INSERT INTO TAREFA
              (TITULO, DESCRICAO, DATA_CADASTRO, DATA_LIMITE, PRIORIDADE)
            VALUES
              (:titulo, :descricao, :data_cadastro, :data_limite, :prioridade)
            ",
            params
        )?;        
        Ok(())
    }

    pub fn delete_task(id_task: i32) -> Result<(), FbError> {
        let mut conn = new_conn().unwrap();
        
        #[derive(Clone, IntoParams)]
        struct Params {
            cod_tarefa: i32,
            controle: i8
        }
        let params = Params {
            cod_tarefa: id_task,
            controle: 1
        };
        let mut sql = 
            "
            DELETE FROM TAREFA
            WHERE COD_TAREFA = :cod_tarefa
            ".to_string();
        let filtrar = true;    
        if filtrar {
            sql += 
                "
                AND 1 = :controle
                ";
        }
        // print!("{}", sql);
        conn.execute(
            &sql,
            params
        )?;
        Ok(())
    }

    pub fn print_tasks_as_tuples() -> Result<(), FbError> {
        // DB Connection
        let mut conn = new_conn().unwrap();
        // Query returning vector of tuples
        let rows: Vec<(i32, String, String)> = conn.query(
            "
            SELECT COD_TAREFA, TITULO, DESCRICAO
            FROM TAREFA
            ORDER BY COD_TAREFA DESC
            ",
            ()
        )?;
        println!("Rows with tuples: {:?}", rows);
        Ok(())
    }

    pub fn print_tasks_as_rows() -> Result<(), FbError> {
        // DB Connection
        let mut conn = new_conn().unwrap();
        // Query returning Row vector
        let rows: Vec<Row> = conn.query(
            "
            SELECT COD_TAREFA, TITULO, DESCRICAO
            FROM TAREFA
            ORDER BY COD_TAREFA DESC
            ",
            ()
        )?;
        for row in &rows {
            for col in &row.cols {
                println!("{}: {:?}", col.name, col.value);
            }
            println!("");
        }
        Ok(())
    }

    pub fn print_last_task() -> Result<(), FbError> {
        // DB Connection
        let mut conn = new_conn().unwrap();
        let rows: Vec<Row> = conn.query(
            "
            SELECT FIRST 1 COD_TAREFA, TITULO, DESCRICAO, DATA_CADASTRO, DATA_LIMITE, PRIORIDADE
            FROM TAREFA
            ORDER BY COD_TAREFA DESC
            ",
            ()
        )?;
        for row in &rows {
            for col in &row.cols {
                println!("{}: {:?}", col.name, col.value);
            }
            println!("");
        }    
        Ok(())
    }
}