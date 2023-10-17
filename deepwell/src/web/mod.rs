/*
 * web/mod.rs
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

mod bytes;
mod connection_type;
mod fetch_direction;
mod file_details;
mod page_details;
mod page_order;
mod provided_value;
mod reference;
mod unwrap;

pub use self::bytes::Bytes;
pub use self::connection_type::ConnectionType;
pub use self::fetch_direction::FetchDirection;
pub use self::file_details::FileDetails;
pub use self::page_details::PageDetails;
pub use self::page_order::{PageOrder, PageOrderColumn};
pub use self::provided_value::ProvidedValue;
pub use self::reference::Reference;
pub use self::unwrap::HttpUnwrap;
