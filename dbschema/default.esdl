module default {
  type DiscordGuild {
    required property guild_id -> int64 {
      constraint exclusive;
    };
    required property name -> str;
    property icon -> str;
    multi link threads -> DiscordThread;
    multi link tags -> Tag;
  }

  type DiscordThread {
    required property thread_id -> int64 {
      constraint exclusive;
    };
    property full_messages_json -> json;
    multi link messages -> DiscordMessage {
      constraint exclusive;
      property order -> int64;
    };
    property create_at -> datetime;
    property markdown_content -> str;
    multi link tags -> Tag;
  }

  type DiscordMessage {
    required property message_id -> int64 {
      constraint exclusive;
    };
    property content -> str;
    property create_at -> datetime;
    property markdown_content -> str;
    multi link tags -> Tag;
  }

  type Tag {
    required property name -> str;
  }
}