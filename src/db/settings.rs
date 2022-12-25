macro_rules! setting {
    ($name:ident ( $ty:ty )) => {
        paste::paste! {
            pub async fn $name() -> anyhow::Result<Option<$ty>> {
                sqlx::query!("SELECT value FROM settings WHERE key = $1", stringify!($name))
                    .fetch_optional(crate::DB.get().await)
                    .await?
                    .map(|r| serde_json::from_str(&r.value))
                    .transpose()
                    .map_err(|e| e.into())
            }

            pub async fn [< set_ $name >](value: $ty) -> anyhow::Result<()> {
                let val = serde_json::to_string(&value)?;

                sqlx::query!(
                    "INSERT INTO settings (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = $2",
                    stringify!($name), val,
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
    setting!(username(String));
    setting!(password(String));
    setting!(session_key(String));
}
