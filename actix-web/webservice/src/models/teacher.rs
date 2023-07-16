use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}


impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(update_teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        }
    }
}

