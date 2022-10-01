module default {
  type DiscordGuild {
    required property guild_id -> int64;
    required property name -> str;
    property icon -> str;
    multi link threads -> DiscordThread;
  }

  type DiscordThread {
    required property thread_channel_id -> int64;
    property full_messages_json -> json;
    multi link messages -> DiscordMessage;
    property create_at -> datetime;
  }

  type DiscordMessage {
    required property message_id -> int64;
    property content -> str;
    property create_at -> datetime;
  }
}