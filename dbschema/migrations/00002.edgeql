CREATE MIGRATION m1ksinxc4ey4bwkpairwdkd2xktrbruf3azjxf75dpvzq42dxbl5ta
    ONTO m1frhunu72db6ffbremwp7weyayzpumctjx5u455fsv5mvaxi27iva
{
  CREATE TYPE default::Tag {
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  ALTER TYPE default::DiscordGuild {
      CREATE MULTI LINK tags -> default::Tag;
  };
  ALTER TYPE default::DiscordMessage {
      CREATE MULTI LINK tags -> default::Tag;
      CREATE PROPERTY markdown_content -> std::str;
  };
  ALTER TYPE default::DiscordThread {
      ALTER LINK messages {
          CREATE CONSTRAINT std::exclusive;
          CREATE PROPERTY order -> std::int64;
      };
  };
  ALTER TYPE default::DiscordThread {
      CREATE MULTI LINK tags -> default::Tag;
  };
  ALTER TYPE default::DiscordThread {
      CREATE PROPERTY markdown_content -> std::str;
  };
};
