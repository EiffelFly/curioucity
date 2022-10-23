CREATE MIGRATION m1lbs4mm7rrws6wq725g2owyv4x45rxs2xdc4go4jbqupal5egusyq
    ONTO initial
{
  CREATE TYPE default::Tag {
      CREATE REQUIRED PROPERTY name -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE SCALAR TYPE default::ResourceType EXTENDING enum<DiscordGuild, DiscordThread, DiscordMessage>;
  CREATE TYPE default::Url {
      CREATE MULTI LINK references -> default::Url {
          CREATE PROPERTY create_at -> std::datetime;
      };
      CREATE REQUIRED PROPERTY resource_type -> default::ResourceType;
      CREATE REQUIRED PROPERTY url -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::DiscordMessage {
      CREATE MULTI LINK tags -> default::Tag;
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
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
      CREATE MULTI LINK messages -> default::DiscordMessage {
          CREATE CONSTRAINT std::exclusive;
          CREATE PROPERTY order -> std::int64;
      };
      CREATE MULTI LINK tags -> default::Tag;
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY create_at -> std::datetime;
      CREATE PROPERTY full_messages_json -> std::json;
      CREATE PROPERTY markdown_content -> std::str;
      CREATE REQUIRED PROPERTY thread_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::DiscordGuild {
      CREATE MULTI LINK tags -> default::Tag;
      CREATE MULTI LINK threads -> default::DiscordThread;
      CREATE LINK url -> default::Url {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY guild_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY icon -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
};
