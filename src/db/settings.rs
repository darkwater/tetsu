macro_rules! setting {
    ($prefix:ident $name:ident ( $ty:ty )) => {
        paste::paste! {
            pub async fn $name() -> anyhow::Result<Option<$ty>> {
                sqlx::query!(
                    "SELECT value FROM settings WHERE key = $1",
                    concat!(stringify!($prefix), "_", stringify!($name)),
                )
                    .fetch_optional(crate::DB.get().await)
                    .await?
                    .map(|r| serde_json::from_str(&r.value))
                    .transpose()
                    .map_err(|e| e.into())
            }

            pub async fn [< set_ $name >](value: $ty) -> anyhow::Result<()> {
                let val = serde_json::to_string(&value)?;

                sqlx::query!(
                    "INSERT INTO settings (key, value)
                    VALUES ($1, $2)
                    ON CONFLICT (key) DO UPDATE SET value = $2",
                    concat!(stringify!($prefix), "_", stringify!($name)),
                    val,
                )
                    .execute(crate::DB.get().await)
                    .await
                    .map(|_| ())
                    .map_err(|e| e.into())
            }
        }
    };
}

pub mod anidb {
    setting!(anidb username(String));
    setting!(anidb password(String));
    setting!(anidb session_key(String));
}

pub mod animebytes {
    setting!(animebytes username(String));
    setting!(animebytes torrentkey(String));
}
