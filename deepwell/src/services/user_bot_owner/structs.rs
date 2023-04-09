/*
 * services/user_bot_owner/structs.rs
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

use crate::services::user::UserProfileOutput;
use crate::web::Reference;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBotUser {
    pub name: String,
    pub email: String,
    pub locale: String,
    pub purpose: String,
    pub owners: Vec<BotOwner>,
    pub bypass_filter: bool,
    pub authorization_token: String, // TODO add authorization token service
                                     // format: [flag]-[uuid]
                                     //         for instance B-1F305167-AE64-4486-809A-09D14659AB4A
                                     //
                                     //         B: create a bot user
                                     //         S: create a site
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBotOwner<'a> {
    pub bot: Reference<'a>,
    pub human: Reference<'a>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBotOwner<'a> {
    pub bot: Reference<'a>,
    pub human: Reference<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotOwner {
    pub user_id: i64,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBotOwnerBody {
    pub description: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotUserOutput {
    #[serde(flatten)]
    pub user: UserProfileOutput,
    pub owners: Vec<BotOwner>,
}
