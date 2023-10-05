#![allow(dead_code)]
use std::collections::HashMap;

use actix_web::{HttpRequest, web, Responder, HttpResponse, get};
use reqwest::{Client,  StatusCode};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::webserver::server::AppState;
// #[tracing::instrument(skip(info, data))]
#[get("/auth")]
pub async fn auth_token(req: HttpRequest ,info:web::Query<QueryAuth>, data: web::Data<AppState>) -> impl Responder {
    if let Some(cookie) = req.cookie("oauth_state") {
        if cookie.value() != info.state {
            return HttpResponse::BadRequest().body("Invalid State");
        }
    } else {
        return HttpResponse::BadRequest().body("Invalid State");
    }
 
    
    println!("{:#?}", info);
    let token_res:AuthResponse = match token_request(data.req_client.clone(), create_token_params(&data.client_id, &data.client_secret, &info.code)).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Error getting token: {:#?}", e);
            return HttpResponse::InternalServerError().body("Error getting token");
        }
    };
        
    
    let validate_res:VerifyResponse = match validate_request(data.pg_pool.clone(),data.req_client.clone(), &token_res.access_token).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Error validating token: {:#?}", e);
            return HttpResponse::InternalServerError().body("Error validating token");
        }
    };
        
        
   


    let record:Result<ReturnSQL, sqlx::Error> =sqlx::query_as(
        r#"
        SELECT * FROM upsert_user($1,$2,$3,$4,$5,$6)
        "#

    )
    .bind(&validate_res.login)
    .bind(&validate_res.user_id)
    .bind(convert_vector_to_string(validate_res.scopes))
    .bind(&token_res.access_token)
    .bind(&token_res.refresh_token)
    .bind(get_times_stamp(token_res.expires_in as i64))
    .fetch_one(&data.pg_pool).await;
     match record {
         Ok(row) => {
            // Handle 
            let mut  map = HashMap::new();
            let oldtoken = row.old_access_token.unwrap().clone();
            map.insert("client_id", data.client_id.as_str());
            map.insert("token", &oldtoken.as_str());
            let _res=    data.req_client.post("https://id.twitch.tv/oauth2/revoke").form(&map).send().await.unwrap();
        }
        Err(e) => {
            tracing::error!("Error inserting user into database: {:?}", e);
            return HttpResponse::InternalServerError().body("Unexpected Error. Please try again later.");
        }
    }

            
            
                
              
                
          
         

   
    //Handle Login 
    //TODO: start listening to chat messages for this user
    //TODO: Set bot to mod for this user
    // println!("{:#?}, {:#?}", token_res,validate_res);

    HttpResponse::Ok().body("Hello")
}

#[derive(Deserialize,Debug)]
pub struct QueryAuth {
     state: String,
     code: String,
     scope: String,
}


#[derive(sqlx::FromRow,Debug)]
struct ReturnSQL{
    success: bool,
    old_name: Option<String>,
    old_scopes: Option<String>,
    old_refresh_token: Option<String>,
    old_access_token: Option<String>,
    old_expires_in: Option<i32>,
    old_created_at: Option<chrono::NaiveDateTime>,
    old_updated_at: Option<chrono::NaiveDateTime>,
}



#[derive(Deserialize,Debug)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    scope: Vec<String>,
    token_type: String,
}
#[derive(Deserialize,Debug)]
pub struct VerifyResponse {
    client_id: String,
    login: String,
    scopes: Vec<String>,
    user_id: String,
    expires_in: u64,
}

 fn create_token_params(client_id: &String, client_secret: & String, code:& String) -> [(&'static str, String); 5 ] {
        //TODO: uri from state 
       [
            ("client_id",client_id.clone()),
            ("client_secret", client_secret.clone()),
            ("code", code.clone()),
            ("grant_type", "authorization_code".to_string()),
            ("redirect_uri", "https://localhost:8080/auth".to_string()),
        ]
   
}

#[tracing::instrument(skip(client))]
async fn token_request(client:Client, params: [(&str, String); 5]) -> Result<AuthResponse, reqwest::Error >{
    let token_res:AuthResponse = client.post("https://id.twitch.tv/oauth2/token").form(&params).send().await?.json().await?;
    println!("{:#?}", token_res);
    Ok(token_res)

}

pub async fn validate_request(pool:Pool<Postgres> ,client:Client, token: &String) -> Result<VerifyResponse, reqwest::Error >{
    let validate_res:VerifyResponse = match client.get("https://id.twitch.tv/oauth2/validate").bearer_auth(token).send().await {
        Ok(res) => res.json().await?,
        Err(e) => {
            match e.status() {
                Some(StatusCode::UNAUTHORIZED) => {
                        let query = sqlx::query!(r#"
                        DELETE FROM users WHERE access_token = $1                       
                        "#, token).execute(&pool).await;
                        match query {
                            Ok(_) =>{}
                            Err(e) => {
                                // TODO: Make so the bot DEPARTS the channel
                                tracing::error!("Error deleting user from database with token: {} Error: {:?}",token ,e);
                            }
                        }
                        return Err(e);
                    
                }
                Some(_) => {
                    return Err(e);
                }
                None => {
                    return Err(e);
                }
            }
         }
            
               
           
        
    };
    Ok(validate_res)

}
fn convert_vector_to_string(vec: Vec<String>) -> String {
    vec.join(",")   
}
fn get_times_stamp(expire_in:i64) ->i32{
    let current_time = chrono::Utc::now().timestamp();
    let expire_time = current_time + expire_in;
    return  expire_time as i32;
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_token_params() {
        let client_id = String::from("test");
        let client_secret = String::from("test");
        let code = String::from("test");
        let params = create_token_params(&client_id, &client_secret, &code);
        assert_eq!(params[0].0, "client_id");
        assert_eq!(params[0].1, "test");
        assert_eq!(params[1].0, "client_secret");
        assert_eq!(params[1].1, "test");
        assert_eq!(params[2].0, "code");
        assert_eq!(params[2].1, "test");
        assert_eq!(params[3].0, "grant_type");
        assert_eq!(params[3].1, "authorization_code");
        assert_eq!(params[4].0, "redirect_uri");
        assert_eq!(params[4].1, "https://localhost:8080/auth");
    }
    #[test]
    fn test_convert_vector_to_string() {
        let vec = vec!["test1".to_string(), "test2".to_string()];
        let string = convert_vector_to_string(vec);
        assert_eq!(string, "test1,test2");
    }
 
}