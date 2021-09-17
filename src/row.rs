use toql::prelude::ToqlError;

use toql::sql_builder::select_stream::Select;
/// Newtype for mysql database row
/// This allows to implement the conversion traits for basic data
/// without violating the orphan rule

pub struct Row();

macro_rules! from_row {
        ($($type:ty),+) => {
            $(
               impl toql::from_row::FromRow<Row, ToqlError> for $type {
               fn forward<'a, I>( iter: &mut I) -> Result<usize,ToqlError>
                where
                        I: Iterator<Item = &'a Select>{
                    if  iter.next().ok_or(
                            toql::error::ToqlError::DeserializeError(
                                toql::deserialize::error::DeserializeError::StreamEnd)
                    )?.is_selected() {
                        Ok(1)
                    } else {
                        Ok(0)
                    }
                }
                // Return None, if unselected or column is null
                fn from_row<'a, I>(
                        _row: &Row,
                        i: &mut usize,
                        iter: &mut I,
                    ) -> std::result::Result<Option<$type>, ToqlError>
                    where
                        I: Iterator<Item = &'a Select> + Clone,
                    {
                        if iter
                       // .inspect(|v| println!("Select is {:?}", v))
                         . next().ok_or(
                            toql::error::ToqlError::DeserializeError(
                                toql::deserialize::error::DeserializeError::StreamEnd)
                         )?.is_selected() {

                            let v = Some(Default::default());
                            *i += 1;
                            Ok(v)
                        } else {
                            Ok(None)
                        }
                    }
                }

            )+
        };
        }

from_row!(
    //NaiveDateTime,
    String, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool
);
