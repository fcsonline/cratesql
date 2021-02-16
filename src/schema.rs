table! {
    api_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Bytea,
        name -> Varchar,
        created_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
        revoked -> Bool,
    }
}

table! {
    background_jobs (id) {
        id -> Int8,
        job_type -> Text,
        data -> Jsonb,
        retries -> Int4,
        last_retry -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    badges (crate_id, badge_type) {
        crate_id -> Int4,
        badge_type -> Varchar,
        attributes -> Jsonb,
    }
}

table! {
    categories (id) {
        id -> Int4,
        category -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        crates_cnt -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    crate_owner_invitations (invited_user_id, crate_id) {
        invited_user_id -> Int4,
        invited_by_user_id -> Int4,
        crate_id -> Int4,
        created_at -> Timestamp,
        token -> Text,
        token_generated_at -> Nullable<Timestamp>,
    }
}

table! {
    crate_owners (crate_id, owner_id, owner_kind) {
        crate_id -> Int4,
        owner_id -> Int4,
        created_at -> Timestamp,
        created_by -> Nullable<Int4>,
        deleted -> Bool,
        updated_at -> Timestamp,
        owner_kind -> Int4,
        email_notifications -> Bool,
    }
}

table! {
    crates (id) {
        id -> Int4,
        name -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        downloads -> Int4,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        documentation -> Nullable<Varchar>,
        readme -> Nullable<Varchar>,
        repository -> Nullable<Varchar>,
        max_upload_size -> Nullable<Int4>,
    }
}

table! {
    crates_categories (crate_id, category_id) {
        crate_id -> Int4,
        category_id -> Int4,
    }
}

table! {
    crates_keywords (crate_id, keyword_id) {
        crate_id -> Int4,
        keyword_id -> Int4,
    }
}

table! {
    dependencies (id) {
        id -> Int4,
        version_id -> Int4,
        crate_id -> Int4,
        req -> Varchar,
        optional -> Bool,
        default_features -> Bool,
        features -> Array<Text>,
        target -> Nullable<Varchar>,
        kind -> Int4,
    }
}

table! {
    emails (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        verified -> Bool,
        token -> Text,
        token_generated_at -> Nullable<Timestamp>,
    }
}

table! {
    follows (user_id, crate_id) {
        user_id -> Int4,
        crate_id -> Int4,
    }
}

table! {
    keywords (id) {
        id -> Int4,
        keyword -> Text,
        crates_cnt -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    metadata (total_downloads) {
        total_downloads -> Int8,
    }
}

table! {
    publish_limit_buckets (user_id) {
        user_id -> Int4,
        tokens -> Int4,
        last_refill -> Timestamp,
    }
}

table! {
    publish_rate_overrides (user_id) {
        user_id -> Int4,
        burst -> Int4,
    }
}

table! {
    readme_renderings (version_id) {
        version_id -> Int4,
        rendered_at -> Timestamp,
    }
}

table! {
    reserved_crate_names (name) {
        name -> Text,
    }
}

table! {
    teams (id) {
        id -> Int4,
        login -> Varchar,
        github_id -> Int4,
        name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        org_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        gh_access_token -> Varchar,
        gh_login -> Varchar,
        name -> Nullable<Varchar>,
        gh_avatar -> Nullable<Varchar>,
        gh_id -> Int4,
        account_lock_reason -> Nullable<Varchar>,
        account_lock_until -> Nullable<Timestamp>,
    }
}

table! {
    version_authors (id) {
        id -> Int4,
        version_id -> Int4,
        name -> Varchar,
    }
}

table! {
    version_downloads (version_id, date) {
        version_id -> Int4,
        downloads -> Int4,
        counted -> Int4,
        date -> Date,
        processed -> Bool,
    }
}

table! {
    version_owner_actions (id) {
        id -> Int4,
        version_id -> Int4,
        user_id -> Int4,
        api_token_id -> Nullable<Int4>,
        action -> Int4,
        time -> Timestamp,
    }
}

table! {
    versions (id) {
        id -> Int4,
        crate_id -> Int4,
        num -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        downloads -> Int4,
        features -> Jsonb,
        yanked -> Bool,
        license -> Nullable<Varchar>,
        crate_size -> Nullable<Int4>,
        published_by -> Nullable<Int4>,
    }
}

table! {
    versions_published_by (version_id) {
        version_id -> Int4,
        email -> Varchar,
    }
}

joinable!(api_tokens -> users (user_id));
joinable!(badges -> crates (crate_id));
joinable!(crate_owner_invitations -> crates (crate_id));
joinable!(crate_owners -> crates (crate_id));
joinable!(crate_owners -> users (created_by));
joinable!(crates_categories -> categories (category_id));
joinable!(crates_categories -> crates (crate_id));
joinable!(crates_keywords -> crates (crate_id));
joinable!(crates_keywords -> keywords (keyword_id));
joinable!(dependencies -> crates (crate_id));
joinable!(dependencies -> versions (version_id));
joinable!(emails -> users (user_id));
joinable!(follows -> crates (crate_id));
joinable!(follows -> users (user_id));
joinable!(publish_limit_buckets -> users (user_id));
joinable!(publish_rate_overrides -> users (user_id));
joinable!(readme_renderings -> versions (version_id));
joinable!(version_authors -> versions (version_id));
joinable!(version_downloads -> versions (version_id));
joinable!(version_owner_actions -> api_tokens (api_token_id));
joinable!(version_owner_actions -> users (user_id));
joinable!(version_owner_actions -> versions (version_id));
joinable!(versions -> crates (crate_id));
joinable!(versions -> users (published_by));
joinable!(versions_published_by -> versions (version_id));

allow_tables_to_appear_in_same_query!(
    api_tokens,
    background_jobs,
    badges,
    categories,
    crate_owner_invitations,
    crate_owners,
    crates,
    crates_categories,
    crates_keywords,
    dependencies,
    emails,
    follows,
    keywords,
    metadata,
    publish_limit_buckets,
    publish_rate_overrides,
    readme_renderings,
    reserved_crate_names,
    teams,
    users,
    version_authors,
    version_downloads,
    version_owner_actions,
    versions,
    versions_published_by,
);
