/*
 * types/maybe.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2025 Wikijump Team
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

/// Denotes that a field is optional in a struct.
///
/// This is meant to be used when doing `UPDATE` operations,
/// since excluding the field entirely is different from setting
/// it to null (`None`).
///
/// The `Unset` variant can only be constructed if the field is absent.
///
/// ## Notes
/// When serializing or deserializing a field using this enum, you must
/// add the following:
/// ```unchecked
/// #[serde(default, skip_serializing_if = "Maybe::is_unset")]
/// ```
///
/// (The `skip_serializing_if` attribute is optional if this is a
/// deserialize-only structure).
///
/// Otherwise you will get an error mentioning that this enum is impossible
/// to serialize.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum Maybe<T> {
    Set(T),

    #[serde(skip)]
    #[default]
    Unset,
}

impl<T> Maybe<T> {
    pub fn to_option(&self) -> Option<&T> {
        match self {
            Maybe::Set(ref value) => Some(value),
            Maybe::Unset => None,
        }
    }

    pub fn is_set(&self) -> bool {
        match self {
            Maybe::Set(_) => true,
            Maybe::Unset => false,
        }
    }

    #[inline]
    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }
}

impl<T> Maybe<T>
where
    T: Into<sea_orm::Value>,
{
    pub fn into_active_value(self) -> sea_orm::ActiveValue<T> {
        match self {
            Maybe::Set(value) => sea_orm::ActiveValue::Set(value),
            Maybe::Unset => sea_orm::ActiveValue::NotSet,
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Option<T> {
        match value {
            Maybe::Set(value) => Some(value),
            Maybe::Unset => None,
        }
    }
}

#[test]
fn serde() {
    use serde_json::json;

    #[derive(Serialize, Deserialize, Debug)]
    struct Object {
        #[serde(default, skip_serializing_if = "Maybe::is_unset")]
        field: Maybe<Option<String>>,
    }

    // Deserialization

    macro_rules! check_deser {
        ($value:expr, $expected:expr $(,)?) => {{
            let object: Object =
                serde_json::from_value($value).expect("Unable to deserialize JSON");

            assert_eq!(
                object.field, $expected,
                "Actual optional item doesn't match expected",
            );
        }};
    }

    check_deser!(json!({}), Maybe::Unset);
    check_deser!(json!({ "field": null }), Maybe::Set(None));
    check_deser!(json!({"field": "apple"}), Maybe::Set(Some(str!("apple"))));

    // Serialization

    macro_rules! check_ser {
        ($field:expr, $expected:expr $(,)?) => {{
            let object = Object { field: $field };
            let json = serde_json::to_string(&object).expect("Unable to serialize JSON");

            assert_eq!(
                json, $expected,
                "Actual generated JSON doesn't match expected",
            );
        }};
    }

    check_ser!(Maybe::Unset, "{}");
    check_ser!(Maybe::Set(None), r#"{"field":null}"#);
    check_ser!(Maybe::Set(Some(str!("banana"))), r#"{"field":"banana"}"#);
}
