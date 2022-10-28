CREATE MIGRATION m1umrfz4i63he4dwm7h63yzh322u6n5vclvk2q55oaqqcg6bru4cnq
    ONTO initial
{
  CREATE TYPE default::Tag {
      CREATE REQUIRED PROPERTY name -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::DiscordGuild {
      CREATE MULTI LINK tags -> default::Tag;
      CREATE REQUIRED PROPERTY guild_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY icon -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  CREATE TYPE default::DiscordThread {
      CREATE MULTI LINK tags -> default::Tag;
      CREATE PROPERTY create_at -> std::datetime;
      CREATE PROPERTY full_messages_json -> std::json;
      CREATE PROPERTY markdown_content -> std::str;
      CREATE REQUIRED PROPERTY thread_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordGuild {
      CREATE MULTI LINK threads -> default::DiscordThread;
  };
  CREATE SCALAR TYPE default::ResourceType EXTENDING enum<DiscordGuild, DiscordThread, DiscordMessage, Website>;
  CREATE TYPE default::Url {
      CREATE MULTI LINK references -> default::Url {
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
  CREATE TYPE default::DiscordMessage {
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE MULTI LINK tags -> default::Tag;
      CREATE PROPERTY content -> std::str;
      CREATE PROPERTY create_at -> std::datetime;
      CREATE PROPERTY markdown_content -> std::str;
      CREATE REQUIRED PROPERTY message_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY order_in_thread -> std::int64;
  };
  ALTER TYPE default::DiscordThread {
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE MULTI LINK messages -> default::DiscordMessage {
          CREATE CONSTRAINT std::exclusive;
          CREATE PROPERTY order -> std::int64;
      };
  };
  ALTER TYPE default::Url {
      CREATE LINK resource := (.<url);
  };
};
