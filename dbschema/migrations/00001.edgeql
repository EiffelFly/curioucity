CREATE MIGRATION m1frhunu72db6ffbremwp7weyayzpumctjx5u455fsv5mvaxi27iva
    ONTO initial
{
  CREATE TYPE default::DiscordMessage {
      CREATE PROPERTY content -> std::str;
      CREATE PROPERTY create_at -> std::datetime;
      CREATE REQUIRED PROPERTY message_id -> std::int64;
  };
  CREATE TYPE default::DiscordThread {
      CREATE MULTI LINK messages -> default::DiscordMessage;
      CREATE PROPERTY create_at -> std::datetime;
      CREATE PROPERTY full_messages_json -> std::json;
      CREATE REQUIRED PROPERTY thread_channel_id -> std::int64;
  };
  CREATE TYPE default::DiscordGuild {
      CREATE MULTI LINK threads -> default::DiscordThread;
      CREATE REQUIRED PROPERTY guild_id -> std::int64;
      CREATE PROPERTY icon -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
};
