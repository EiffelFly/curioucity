import * as urlServices from "./grpc_url.js";
import * as tagServices from "./grpc_tag.js";
import * as discordServices from "./grpc_discord.js";

export const GRPC_API_HOST = "localhost:8080";

const grpc = () => {
  urlServices.listUrl();
  urlServices.createUrl();
  urlServices.deleteUrl();
  urlServices.getUrl();
  tagServices.createTag();
  tagServices.deleteTag();
  discordServices.listDiscordGuild();
  discordServices.createDiscordGuild();
  discordServices.deleteDiscordGuild();
  discordServices.getDiscordGuild();
  discordServices.listDiscordThread();
  discordServices.createDiscordThread();
  discordServices.deleteDiscordThread();
  discordServices.getDiscordThread();
  discordServices.listDiscordMessage();
  discordServices.createDiscordMessage();
  discordServices.deleteDiscordMessage();
  discordServices.getDiscordMessage();
};

export default grpc;
