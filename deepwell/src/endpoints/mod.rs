/*
 * endpoints/endpoints.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Definitions of endpoints invoked by different API routes.
//!
//! The structure of the internal API is specified in `api.rs`, not here.
//! This module contains implementations of those endpoints only.
//!
//! The module should not contain core business logic of its own, which should
//! instead live in `services`. Endpoint definitions should ideally be wrappers
//! around service calls, or possibly perform modest data conversion for HTTP.

mod prelude {
    pub use crate::api::{ApiRequest, ApiResponse};
    pub use crate::services::{
        AliasService, BlobService, CategoryService, DomainService, Error as ServiceError,
        FileRevisionService, FileService, InteractionService, LinkService,
        MessageService, MfaService, PageRevisionService, PageService, ParentService,
        RenderService, RequestFetchService, ScoreService, ServiceContext, SessionService,
        SiteService, TextService, UserService, ViewService, VoteService,
    };
    pub use crate::utils::error_response;
    pub use crate::web::HttpUnwrap;
    pub use sea_orm::{ConnectionTrait, TransactionTrait};
    pub use std::convert::TryFrom;
    pub use tide::{Body, Error as TideError, Request, Response, StatusCode};

    use serde::Serialize;

    pub fn build_json_response<T: Serialize>(
        data: &T,
        status: StatusCode,
    ) -> ApiResponse {
        let body = Body::from_json(data)?;
        let response = Response::builder(status).body(body).into();
        Ok(response)
    }
}

pub mod auth;
pub mod category;
pub mod email;
pub mod file;
pub mod file_revision;
pub mod link;
pub mod locale;
pub mod misc;
pub mod page;
pub mod page_revision;
pub mod parent;
pub mod site;
pub mod site_member;
pub mod text;
pub mod user;
pub mod user_bot;
pub mod view;
pub mod vote;
