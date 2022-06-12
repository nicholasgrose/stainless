CREATE TABLE server_configs (
    name VARCHAR(200) REFERENCES server_types(name) NOT NULL,

    PRIMARY KEY (name)
);

CREATE TABLE server_types (
    name VARCHAR(200) NOT NULL,
    server_type VARCHAR(50) NOT NULL,

    FOREIGN KEY (name) REFERENCES server_configs(name),
    PRIMARY KEY (name, server_type)
);

CREATE TABLE minecraft_servers (
    name VARCHAR(200) NOT NULL,
    server_type VARCHAR(50) GENERATED ALWAYS AS ('Minecraft') STORED NOT NULL,
    game_version VARCHAR(50) NOT NULL,

    FOREIGN KEY (name, server_type) REFERENCES server_types(name, server_type),
    FOREIGN KEY (name) REFERENCES server_configs(name),
    FOREIGN KEY (name) REFERENCES minecraft_jvm_arguments(name),
    FOREIGN KEY (name) REFERENCES minecraft_types(name),
    PRIMARY KEY (name)
);

CREATE TABLE minecraft_jvm_arguments (
    name VARCHAR(200) NOT NULL,
    argument VARCHAR(200) NOT NULL,

    FOREIGN KEY (name) REFERENCES minecraft_servers(name),
    PRIMARY KEY (name, argument)
);

CREATE TABLE minecraft_types (
    name VARCHAR(200) NOT NULL,
    minecraft_server_type VARCHAR(50) NOT NULL,

    FOREIGN KEY (name) REFERENCES minecraft_servers(name),
    PRIMARY KEY (name, minecraft_server_type)
);

CREATE TABLE papermc_servers (
    name VARCHAR(200) NOT NULL,
    minecraft_server_type VARCHAR(50) GENERATED ALWAYS AS ('PaperMC') STORED NOT NULL,
    project VARCHAR(50) NOT NULL,
    build INT NOT NULL,

    CHECK (build > 0),

    FOREIGN KEY (name) REFERENCES minecraft_servers(name),
    FOREIGN KEY (name, minecraft_server_type) REFERENCES minecraft_types(name, minecraft_server_type),
    PRIMARY KEY (name)
);
