table! {
    hero (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}
