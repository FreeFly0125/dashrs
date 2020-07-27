use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod internal {
    use crate::{
        model::creator::Creator,
        serde::{HasRobtopFormat, IndexedDeserializer, IndexedSerializer},
        DeError, SerError,
    };
    use serde::{Deserialize, Serialize};
    use std::{borrow::Cow, io::Write};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct InternalCreator<'a> {
        pub user_id: u64,

        #[serde(borrow)]
        pub name: &'a str,

        #[serde(with = "crate::util::default_to_none")]
        pub account_id: Option<u64>,
    }

    impl<'a> HasRobtopFormat<'a> for Creator<'a> {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError> {
            let internal = InternalCreator::deserialize(&mut IndexedDeserializer::new(input, ":", false))?;

            Ok(Creator {
                user_id: internal.user_id,
                name: Cow::Borrowed(internal.name),
                account_id: internal.account_id,
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalCreator {
                user_id: self.user_id,
                name: self.name.as_ref(),
                account_id: self.account_id,
            };

            internal.serialize(&mut IndexedSerializer::new(":", writer, false))
        }
    }
}

/// Struct modelling a [`Creator`] of a level.
///
/// ## GD Internals
/// The Geometry Dash servers provide a list of the creators of the
/// levels in a `getGJLevels` response.
///
/// Creators do not use the map-like representation, meaning the order of fields in the raw data
/// must correspond to the order of fields in this struct.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Creator<'a> {
    /// The [`Creator`]'s unique user ID
    pub user_id: u64,

    /// The [`Creator`]'s name
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    /// The [`Creator`]'s unique account ID.
    ///
    /// This field is [`None`] if the creator hasn't registered for an account.
    pub account_id: Option<u64>,
}

impl<'a> Creator<'a> {
    pub fn into_owned(self) -> Creator<'static> {
        Creator {
            user_id: self.user_id,
            name: Cow::Owned(self.name.into_owned()),
            account_id: self.account_id,
        }
    }
}
