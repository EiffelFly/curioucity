CREATE MIGRATION m1rsdiu3xnjpvpy4gszae3w6ypbpi3gviisdvz4tkdkplkipuxf3ma
    ONTO m1ksinxc4ey4bwkpairwdkd2xktrbruf3azjxf75dpvzq42dxbl5ta
{
  ALTER TYPE default::DiscordGuild {
      ALTER PROPERTY guild_id {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordMessage {
      ALTER PROPERTY message_id {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::DiscordThread {
      ALTER PROPERTY thread_channel_id {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
