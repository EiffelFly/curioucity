import * as urlServices from "./grpc_url.js";
import * as tagServices from "./grpc_tag.js";
import * as discordServices from "./grpc_discord.js";

export const GRPC_API_HOST = "localhost:8080";

const grpc = () => {
  // urlServices.createUrl();
  // urlServices.deleteUrl();
  // urlServices.getUrl();
  // urlServices.listUrl();
  // tagServices.createTag();
  // tagServices.deleteTag();
  // discordServices.createDiscordGuild();
  // discordServices.deleteDiscordGuild();
  // discordServices.getDiscordGuild();
  // discordServices.listDiscordGuild();
  // discordServices.createDiscordThread();
  // discordServices.deleteDiscordThread();
  // discordServices.getDiscordThread();
  // discordServices.listDiscordThread();
  discordServices.createDiscordMessage();
  discordServices.deleteDiscordMessage();
};

export default grpc;
