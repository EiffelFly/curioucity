CREATE MIGRATION m1r2ugjkp5e5smrqty7qjxuu2vklagvlqnukt5fwltoh2wgze35qca
    ONTO m1s7ynu4odlu2iftaufaa7g6yinvsfonbnlamtotbukn77nkkobozq
{
  ALTER SCALAR TYPE default::ResourceType EXTENDING enum<DiscordGuild, DiscordThread, DiscordMessage, BlogPost>;
};
