CREATE MIGRATION m1jp74qyh3i7mzz5c5vmluw7o4meuzxbmb3wwivjgscz5uuczsdrua
    ONTO m1jgmqhuzr634rnjocqthacnbminfdh3vba3ccx7sxdf4ij3qmkcjq
{
  CREATE TYPE default::Url {
      CREATE MULTI LINK references -> default::Url {
          CREATE PROPERTY create_at -> std::datetime;
      };
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
};
