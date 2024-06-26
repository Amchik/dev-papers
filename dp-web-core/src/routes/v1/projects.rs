use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, put},
    Json, Router,
};
use dp_core::v1::{
    api,
    endpoint::{
        projects::{
            CreateProject, CreateProjectBody, DeleteProject, ListProjects, ProjectInfo,
            ProjectListQuery, ProjectPath,
        },
        Endpoint,
    },
    project::ProjectTy,
};

use crate::routes::AppState;

use super::models::user::AuthorizedUser;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route(ListProjects::partial_path(), get(list_projects))
        .route(CreateProject::partial_path(), put(create_project))
        .route(DeleteProject::partial_path(), delete(delete_project))
}

pub async fn list_projects(
    State(AppState { db, .. }): State<AppState>,
    AuthorizedUser { user, .. }: AuthorizedUser,
    Query(ProjectListQuery { limit, skip }): Query<<ListProjects as Endpoint>::Query>,
) -> api::Response<<ListProjects as Endpoint>::Response> {
    let limit = match limit {
        0 => 50,
        v @ 1..=50 => v,
        _ => return api::Response::error(api::Error::InvalidInput),
    };
    let (start, stop) = (limit * skip, limit * skip + limit);

    let list = sqlx::query!(
        "select * from project where author_id = ? limit ?,?",
        user.id,
        start,
        stop
    )
    .fetch_all(&db)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|v| ProjectInfo {
        id: v.id,
        ty: ProjectTy::from_bits(v.ty),
        title: v.title,
        description: v.descript,
        author_id: v.author_id,
    })
    .collect();

    api::Response::Success(list)
}

pub async fn create_project(
    State(AppState { db, .. }): State<AppState>,
    AuthorizedUser { user, .. }: AuthorizedUser,
    Json(CreateProjectBody { title, description }): Json<<CreateProject as Endpoint>::Body>,
) -> api::Response<<CreateProject as Endpoint>::Response, &'static str> {
    let ty = ProjectTy::Legacy;
    let ity = ty as i64;

    if !matches!(title.len(), 2..=40) {
        return api::Response::error_description(
            api::Error::InvalidInput,
            "lenght of `title` should be in range 2..=40",
        );
    }

    let id = sqlx::query!(
        "insert into project(ty,title,descript,author_id) values(?,?,?,?)",
        ity,
        title,
        description,
        user.id
    )
    .execute(&db)
    .await
    .map(|v| v.last_insert_rowid());

    match id {
        Ok(id) => api::Response::Success(ProjectInfo {
            id,
            ty,
            title,
            description,
            author_id: user.id,
        }),
        Err(_) => api::Response::error(api::Error::Conflict),
    }
}

pub async fn delete_project(
    AuthorizedUser { user, .. }: AuthorizedUser,
    Path(ProjectPath { id }): Path<ProjectPath>,
    State(AppState { db, .. }): State<AppState>,
) -> api::Response {
    let res = sqlx::query!(
        "delete from project where id = ? and author_id = ?",
        id,
        user.id
    )
    .execute(&db)
    .await
    .map(|v| v.rows_affected())
    .unwrap_or_default();

    if res == 0 {
        api::Response::error(api::Error::Forbidden)
    } else {
        api::Response::Success(api::EmptyErrorData)
    }
}
