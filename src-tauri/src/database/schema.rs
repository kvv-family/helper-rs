diesel::table! {
    jobs (id) {
        id -> Integer,
        title -> Text,
        comment -> Text,
        date_start -> Text,
        time_start -> Text,
        date_end -> Text,
        time_end -> Text,
    }
}
