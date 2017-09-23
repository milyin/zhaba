infer_schema!("dotenv:DATABASE_URL");

// views are not inferred automatically
table! {
    user_info (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}
