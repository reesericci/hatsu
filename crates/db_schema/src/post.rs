//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub object: String,
    pub attributed_to: String,
    pub in_reply_to: Option<String>,
    pub in_reply_to_root: Option<String>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub last_refreshed_at: String,
    pub local: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AttributedTo",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_one = "super::user_feed_item::Entity")]
    UserFeedItem,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Id",
        to = "Column::InReplyToRoot"
    )]
    /// <https://www.sea-ql.org/SeaORM/docs/relation/self-referencing/>
    SelfReferencing,
}

/// <https://www.sea-ql.org/SeaORM/docs/relation/self-referencing/>
pub struct SelfReferencingLink;

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::user_feed_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserFeedItem.def()
    }
}

/// <https://www.sea-ql.org/SeaORM/docs/relation/self-referencing/>
impl Linked for SelfReferencingLink {
    type FromEntity = Entity;
    type ToEntity = Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![Relation::SelfReferencing.def()]
    }
}
