use chrono::Local;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use log::debug;

use crate::api::exceptions::internal_server_error;
use crate::api::exceptions::AppError;
use crate::todo::db_schema::labels as tbl_labels;
use crate::todo::db_schema::todos as tbl_todos;
use crate::todo::models::Label;
use crate::todo::schemas::NewLabel;

use super::models::Todo;
use super::schemas::TodoCreate;
use super::schemas::TodoUpdate;

pub struct TodoService {
    pub conn: Object,
}

impl TodoService {
    pub async fn get_label(&mut self, label_id: i32) -> Result<Label, AppError> {
        debug!("Fetching label from db.");
        let res = self
            .conn
            .interact(move |conn| tbl_labels::table.find(label_id).first(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }
    pub async fn list_labels(&mut self) -> Result<Vec<Label>, AppError> {
        debug!("Fetching labels from db.");
        let res = self
            .conn
            .interact(|conn| tbl_labels::table.select(Label::as_select()).load(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn create_label(&mut self, label: NewLabel) -> Result<Label, AppError> {
        debug!("Creating label for a todo.");
        let res: Label = self
            .conn
            .interact(|conn| {
                diesel::insert_into(tbl_labels::table)
                    .values(label)
                    .returning(Label::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        debug!("Label created.");
        Ok(res)
    }

    pub async fn update_label(&mut self, pk: i32, payload: NewLabel) -> Result<Label, AppError> {
        debug!("Updating label with ID: {}.", pk);
        let res: Label = self
            .conn
            .interact(move |conn| {
                diesel::update(tbl_labels::table.filter(tbl_labels::id.eq(pk)))
                    .set((
                        tbl_labels::name.eq(payload.name),
                        tbl_labels::updated_at.eq(Local::now().naive_local()),
                    ))
                    .returning(Label::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn delete_label(&mut self, pk: i32) -> Result<(), AppError> {
        debug!("Deleting label with ID: {}.", pk);
        let res: usize = self
            .conn
            .interact(move |conn| {
                diesel::delete(tbl_labels::table.filter(tbl_labels::id.eq(pk)))
                    .execute(conn)
                    .expect("Error deleting the label.")
            })
            .await
            .map_err(internal_server_error)?;
        if res > 0 {
            return Ok(());
        }
        return Err(AppError::NotFound);
    }

    pub async fn get_todo(&mut self, todo_id: i32) -> Result<Todo, AppError> {
        debug!("Fetching todo from db.");
        let res = self
            .conn
            .interact(move |conn| tbl_todos::table.find(todo_id).first(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }
    pub async fn list_todos(&mut self) -> Result<Vec<Todo>, AppError> {
        debug!("Fetching todos from db.");
        let res = self
            .conn
            .interact(|conn| tbl_todos::table.select(Todo::as_select()).load(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn create_todo(&mut self, payload: TodoCreate) -> Result<Todo, AppError> {
        debug!("Creating todo.");
        let res: Todo = self
            .conn
            .interact(|conn| {
                diesel::insert_into(tbl_todos::table)
                    .values(payload)
                    .returning(Todo::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        debug!("Todo created.");
        Ok(res)
    }

    pub async fn update_todo(&mut self, pk: i32, payload: TodoUpdate) -> Result<Todo, AppError> {
        debug!("Updating todo with ID: {}.", pk);
        let res: Todo = self
            .conn
            .interact(move |conn| {
                diesel::update(tbl_todos::table.filter(tbl_todos::id.eq(pk)))
                    .set((
                        tbl_todos::title.eq(payload.title),
                        tbl_todos::description.eq(payload.description),
                        tbl_todos::label_id.eq(payload.label_id),
                        tbl_todos::updated_at.eq(Local::now().naive_local()),
                    ))
                    .returning(Todo::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn delete_todo(&mut self, pk: i32) -> Result<(), AppError> {
        debug!("Deleting todo with ID: {}.", pk);
        let res: usize = self
            .conn
            .interact(move |conn| {
                diesel::delete(tbl_todos::table.filter(tbl_todos::id.eq(pk)))
                    .execute(conn)
                    .expect("Error deleting the todo.")
            })
            .await
            .map_err(internal_server_error)?;
        if res > 0 {
            return Ok(());
        }
        return Err(AppError::NotFound);
    }
}
