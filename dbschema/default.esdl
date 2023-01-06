module default {
  type DiscordGuild {
    required property kind -> str {
      constraint one_of("DISCORD_GUILD");
    };
    required property guild_id -> int64 {
      constraint exclusive;
    };
    required property name -> str;
    required property created_timestamp_at_discord -> datetime;
    required property created_timestamp_at_curioucity -> datetime;
    property icon -> str;
    multi link threads -> DiscordThread;
    multi link tags -> Tag {
      on target delete allow;
    };
    link url -> Url {
      constraint exclusive;
    };
  }

  type DiscordThread {
    required property kind -> str {
      constraint one_of("DISCORD_THREAD");
    };
    required property thread_id -> int64 {
      constraint exclusive;
    };
    required property created_timestamp_at_discord -> datetime;
    required property created_timestamp_at_curioucity -> datetime;
    property full_messages_json -> json;
    multi link messages -> DiscordMessage {
      constraint exclusive;
      property order -> int64;
    };
    property markdown_content -> str;
    multi link tags -> Tag {
      on target delete allow;
    };
    link url -> Url {
      constraint exclusive;
    };
  }

  type DiscordMessage {
    required property kind -> str {
      constraint one_of("DISCORD_MESSAGE");
    };
    required property message_id -> int64 {
      constraint exclusive;
    };
    required property order_in_thread -> int64;
    required property created_timestamp_at_discord -> datetime;
    required property created_timestamp_at_curioucity -> datetime;
    property content -> str;
    property markdown_content -> str;
    multi link tags -> Tag {
      on target delete allow;
    };
    link url -> Url {
      constraint exclusive;
    };
  }

  type Tag {
    required property name -> str {
      constraint exclusive;
    };
    required property created_timestamp_at_curioucity -> datetime;
    multi link resources := .<tags
  }

  scalar type ResourceType extending enum<DiscordGuild, DiscordThread, DiscordMessage, Website>;

  type Url {
    required property url -> str {
      constraint exclusive;
    }
    required property resource_type -> ResourceType;
    required property created_timestamp_at_curioucity -> datetime;
    link resource := .<url;
    multi link references -> Url {
      property created_at -> datetime;
      on target delete allow;
    }
  }
}