CREATE MIGRATION m17f5q5dvzwvetpnjnjn2k75zob7pb4653fxbyvm2fuwy64xvhdmpq
    ONTO initial
{
  CREATE TYPE default::Tag {
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  CREATE TYPE default::DiscordMessage {
      CREATE MULTI LINK tags -> default::Tag;
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
      CREATE REQUIRED PROPERTY guild_id -> std::int64 {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY icon -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
};
