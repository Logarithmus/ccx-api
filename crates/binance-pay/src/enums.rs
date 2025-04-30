#[cfg(feature = "db")]
#[macro_export]
macro_rules! enum_diesel_sql {
    ($name:ident) => {
        impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for $name {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
            ) -> diesel::serialize::Result {
                diesel::serialize::ToSql::<diesel::sql_types::Text, diesel::pg::Pg>::to_sql(
                    &self.name(),
                    &mut out.reborrow(),
                )
            }
        }

        impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for $name {
            fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
                let name = <String as diesel::deserialize::FromSql<
                    diesel::sql_types::Text,
                    diesel::pg::Pg,
                >>::from_sql(bytes)?;

                Self::from_name(name.as_str()).ok_or_else(|| {
                    format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
                })
            }
        }
    };

    ($name:ident, $sql_type:ty) => {
        impl diesel::serialize::ToSql<$sql_type, diesel::pg::Pg> for $name {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
            ) -> diesel::serialize::Result {
                use std::io::Write;

                out.write_all(self.name().as_bytes())?;
                Ok(diesel::serialize::IsNull::No)
            }
        }

        impl diesel::deserialize::FromSql<$sql_type, diesel::pg::Pg> for $name {
            fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
                let name = std::str::from_utf8(bytes.as_bytes()).map_err(|_| "Invalid UTF-8")?;
                Self::from_name(name).ok_or_else(|| {
                    format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
                })
            }
        }
    };
}

#[macro_export]
macro_rules! enum_from_name {
    ($name:ident) => {
        impl $name {
            #[allow(unused)]
            pub fn name(&self) -> String
            where
                Self: serde::Serialize,
            {
                serde_plain::to_string(&self).expect("encode enum variant")
            }

            #[allow(unused)]
            pub fn from_name<'a>(value: &'a str) -> Option<Self>
            where
                Self: serde::Deserialize<'a>,
            {
                serde_plain::from_str(value).ok()
            }
        }

        impl std::str::FromStr for $name {
            type Err = serde_plain::Error;

            fn from_str(s: &str) -> std::result::Result<$name, Self::Err> {
                serde_plain::from_str(s)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&serde_plain::to_string(self).unwrap())
            }
        }
    };
}

#[cfg(test)]
mod test {
    use enum_from_name;

    #[test]
    fn test_enum_from_name() {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        enum TestEnum {
            Variant1,
            Variant2,
            SomeLongVariant,
        }
        enum_from_name!(TestEnum);

        assert_eq!(TestEnum::Variant1, TestEnum::from_name("Variant1").unwrap());
        assert_eq!(TestEnum::Variant2.name(), "Variant2".to_string());
    }
}
