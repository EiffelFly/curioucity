CREATE MIGRATION m1b5qsnv2etukvjsovq3ircnru46jsmd4qwwlba4azj2j53csa4qea
    ONTO initial
{
  CREATE TYPE default::DiscordGuild {
      CREATE REQUIRED PROPERTY guild_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY icon -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  CREATE TYPE default::Tag {
      CREATE REQUIRED PROPERTY name -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordGuild {
      CREATE MULTI LINK tags -> default::Tag {
          ON TARGET DELETE ALLOW;
      };
  };
  CREATE TYPE default::DiscordMessage {
      CREATE MULTI LINK tags -> default::Tag {
          ON TARGET DELETE ALLOW;
      };
      CREATE PROPERTY content -> std::str;
      CREATE PROPERTY create_at -> std::datetime;
      CREATE PROPERTY markdown_content -> std::str;
      CREATE REQUIRED PROPERTY message_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY order_in_thread -> std::int64;
  };
  CREATE TYPE default::DiscordThread {
      CREATE MULTI LINK tags -> default::Tag {
          ON TARGET DELETE ALLOW;
      };
      CREATE MULTI LINK messages -> default::DiscordMessage {
          CREATE CONSTRAINT std::exclusive;
          CREATE PROPERTY order -> std::int64;
      };
      CREATE PROPERTY create_at -> std::datetime;
      CREATE PROPERTY full_messages_json -> std::json;
      CREATE PROPERTY markdown_content -> std::str;
      CREATE REQUIRED PROPERTY thread_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::Tag {
      CREATE MULTI LINK resources := (.<tags);
  };
  ALTER TYPE default::DiscordGuild {
      CREATE MULTI LINK threads -> default::DiscordThread;
  };
  CREATE SCALAR TYPE default::ResourceType EXTENDING enum<DiscordGuild, DiscordThread, DiscordMessage, Website>;
  CREATE TYPE default::Url {
      CREATE MULTI LINK references -> default::Url {
          ON TARGET DELETE ALLOW;
          CREATE PROPERTY create_at -> std::datetime;
      };
      CREATE REQUIRED PROPERTY resource_type -> default::ResourceType;
      CREATE REQUIRED PROPERTY url -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordGuild {
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordMessage {
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordThread {
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::Url {
      CREATE LINK resource := (.<url);
  };
};
