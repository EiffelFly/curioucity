import * as urlServices from "./grpc_url.js";
import * as tagServices from "./grpc_tag.js";
import * as discordGuildServices from "./grpc_discord_guild.js";
import * as discordThreadServices from "./grpc_discord_thread.js";
import * as discordMessageServices from "./grpc_discord_message.js";

export const GRPC_API_HOST = "localhost:8080";

const grpc = () => {
  // urlServices.listUrl();
  // urlServices.createUrl();
  // urlServices.deleteUrl();
  // urlServices.getUrl();
  // urlServices.cleanUpUrls();
  // tagServices.listTag();
  // tagServices.getTag();
  // tagServices.createTag();
  // tagServices.deleteTag();
  // tagServices.cleanUpTags();
  // discordGuildServices.listDiscordGuild();
  // discordGuildServices.createDiscordGuild();
  // discordGuildServices.deleteDiscordGuild();
  // discordGuildServices.getDiscordGuild();
  // discordGuildServices.cleanupDiscordGuild();
  discordThreadServices.listDiscordThread();
  discordThreadServices.createDiscordThread();
  discordThreadServices.deleteDiscordThread();
  discordThreadServices.getDiscordThread();
  discordThreadServices.cleanupDiscordThread();
};

export default grpc;
