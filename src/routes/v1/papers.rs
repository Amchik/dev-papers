// use axum::{
//     extract::{Path, Query, State},
//     http::StatusCode,
//     response::IntoResponse,
// };
// use serde::{Deserialize, Serialize};
// use tokio::fs;

// use crate::routes::AppState;

// use super::api;

// #[derive(Serialize)]
// pub struct Paper {
//     id: i64,
//     title: String,
//     author_str: String,
// }

// #[derive(Deserialize)]
// pub struct PapersQuery {
//     start: Option<u32>,
// }

// pub async fn papers_list(
//     Query(PapersQuery { start }): Query<PapersQuery>,
//     State(AppState { db, .. }): State<AppState>,
// ) -> api::Response<Vec<Paper>> {
//     api::Response::Error(api::Error::Obsolete)
//     // let skip = start.unwrap_or_default();
//     // let skips = skip + 25;
//     // let query = sqlx::query!("SELECT * FROM papers LIMIT ?, ?;", skip, skips)
//     //     .fetch_all(&db)
//     //     .await;
//     // match query {
//     //     Ok(query) => api::Response::Success(
//     //         query
//     //             .into_iter()
//     //             .map(|v| Paper {
//     //                 id: v.id,
//     //                 title: v.title,
//     //                 author_str: v.author_str,
//     //             })
//     //             .collect(),
//     //     ),
//     //     _ => api::Response::Error(api::Error::NotFound),
//     // }
// }

// pub async fn get_paper(
//     Path(id): Path<i64>,
//     State(AppState { db, .. }): State<AppState>,
// ) -> api::Response<Paper> {
//     api::Response::Error(api::Error::Obsolete)
//     // let query = sqlx::query!("SELECT * FROM papers WHERE id = ? LIMIT 1;", id)
//     //     .fetch_one(&db)
//     //     .await;
//     // match query {
//     //     Ok(query) => api::Response::Success(Paper {
//     //         id,
//     //         title: query.title,
//     //         author_str: query.author_str,
//     //     }),
//     //     _ => api::Response::Error(api::Error::NotFound),
//     // }
// }
// pub async fn download_paper(
//     Path(id): Path<i64>,
//     State(AppState { papers_path, db }): State<AppState>,
// ) -> impl IntoResponse {
//     api::Response::<()>::Error(api::Error::Obsolete)
//     // let query = sqlx::query!("SELECT * FROM papers WHERE id = ? LIMIT 1;", id)
//     //     .fetch_one(&db)
//     //     .await;
//     // match query {
//     //     Ok(query) => {
//     //         let Ok(data) = fs::read(papers_path.join(query.path)).await else {
//     //             panic!("failed to read pdf");
//     //         };

//     //         (StatusCode::OK, data).into_response()
//     //     }
//     //     _ => api::EmptyResponse::Error(api::Error::NotFound).into_response(),
//     // }
// }
