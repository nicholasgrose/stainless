// @generated automatically by Diesel CLI.

diesel::table! {
    minecraft_jvm_arguments (name, argument) {
        name -> Text,
        argument -> Text,
    }
}

diesel::table! {
    minecraft_servers (name) {
        name -> Text,
        server_type -> Text,
        game_version -> Text,
    }
}

diesel::table! {
    minecraft_types (name, minecraft_server_type) {
        name -> Text,
        minecraft_server_type -> Text,
    }
}

diesel::table! {
    papermc_servers (name) {
        name -> Text,
        minecraft_server_type -> Text,
        project -> Text,
        build -> Integer,
    }
}

diesel::table! {
    server_configs (name) {
        name -> Text,
    }
}

diesel::table! {
    server_types (name, server_type) {
        name -> Text,
        server_type -> Text,
    }
}

diesel::joinable!(minecraft_servers -> server_configs (name));
diesel::joinable!(papermc_servers -> minecraft_servers (name));

diesel::allow_tables_to_appear_in_same_query!(
    minecraft_jvm_arguments,
    minecraft_servers,
    minecraft_types,
    papermc_servers,
    server_configs,
    server_types,
);
