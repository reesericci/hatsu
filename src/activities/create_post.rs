// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/activities/create_post.rs

use std::env;

use activitypub_federation::{
  activity_queue::send_activity,
  config::Data,
  fetch::object_id::ObjectId,
  kinds::activity::CreateType,
  protocol::{
    helpers::deserialize_one_or_many,
    context::WithContext
  },
  traits::{ActivityHandler, Object},
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
  AppData,
  entities::{
    prelude::*,
    post::Model as DbPost,
    user::Model as DbUser,
  },
  error::AppError,
  objects::post::Note,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
  pub(crate) actor: ObjectId<DbUser>,
  #[serde(deserialize_with = "deserialize_one_or_many")]
  pub(crate) to: Vec<Url>,
  pub(crate) object: Note,
  #[serde(rename = "type")]
  pub(crate) kind: CreateType,
  pub(crate) id: Url,
}

impl CreatePost {
  pub async fn send(note: Note, inbox: Url, data: &Data<AppData>) -> Result<(), AppError> {
    tracing::info!("Sending reply to {}", &note.attributed_to);

    let create = CreatePost {
      actor: note.attributed_to.clone(),
      to: note.to.clone(),
      object: note,
      kind: CreateType::Create,
      // TODO: I Don't Know
      id: Url::parse(&format!("https://{}/o/{}", data.domain(), Uuid::new_v4()))?.into(),
    };
    let create_with_context = WithContext::new_default(create);

    // TODO: multiple user
    let db_user: DbUser = User::find_by_id(env::var("HATSU_TEST_ACCOUNT")?)
        .one(&data.conn)
        .await?
        .unwrap();

    send_activity(create_with_context, &db_user, vec![inbox], data).await?;

    Ok(())
  }
}

#[async_trait::async_trait]
impl ActivityHandler for CreatePost {
  type DataType = AppData;
  type Error = AppError;

  fn id(&self) -> &Url {
    &self.id
  }

  fn actor(&self) -> &Url {
    self.actor.inner()
  }

  async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
    DbPost::verify(&self.object, &self.id, data).await?;
    Ok(())
  }

  async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
    DbPost::from_json(self.object, data).await?;
    Ok(())
  }
}