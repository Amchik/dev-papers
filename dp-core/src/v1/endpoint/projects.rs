use serde::{Deserialize, Serialize};

use crate::v1::project::ProjectTy;

use super::{Endpoint, HTTPMethod};

pub const PREFIX: &'static str = "/projects";

#[derive(Serialize, Deserialize)]
pub struct CreateProjectBody {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectInfo {
    pub id: i64,
    pub ty: ProjectTy,
    pub title: String,
    pub description: Option<String>,
    pub author_id: i64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectListQuery {
    #[serde(default)]
    pub limit: u32,
    #[serde(default)]
    pub skip: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectPath {
    pub id: i64,
}

pub struct ListProjects;
impl Endpoint for ListProjects {
    type Body = ();
    type Query = ProjectListQuery;
    type Response = Vec<ProjectInfo>;

    fn method() -> HTTPMethod {
        HTTPMethod::Get
    }
    fn partial_path() -> &'static str {
        "/"
    }
    fn build_path(&self) -> String {
        PREFIX.to_owned()
    }
}

pub struct CreateProject;
impl Endpoint for CreateProject {
    type Body = CreateProjectBody;
    type Query = ();
    type Response = ProjectInfo;

    fn method() -> HTTPMethod {
        HTTPMethod::Put
    }
    fn partial_path() -> &'static str {
        "/"
    }
    fn build_path(&self) -> String {
        PREFIX.to_owned()
    }
}

pub struct DeleteProject(pub ProjectPath);
impl Endpoint for DeleteProject {
    type Body = ();
    type Query = ();
    type Response = ();

    fn method() -> HTTPMethod {
        HTTPMethod::Delete
    }
    fn partial_path() -> &'static str {
        "/:id"
    }
    fn build_path(&self) -> String {
        format!("{PREFIX}/{}", self.0.id)
    }
}
