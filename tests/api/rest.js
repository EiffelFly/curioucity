import * as urlServices from "./rest_url.js";
import * as tagServices from "./rest_tag.js";
import * as discordMessageServices from "./rest_discord_message.js";
import * as discordThreadServices from "./rest_discord_thread.js";
import * as discordGuildServices from "./rest_discord_guild.js";

export const options = {};

export const API_HOST = "http://localhost:8080";

const main = () => {
  // urlServices.createUrl();
  // urlServices.deleteUrl();
  // urlServices.getUrl();
  // urlServices.listUrl();
  // urlServices.cleanUpUrl();
  // tagServices.createTag();
  // tagServices.deleteTag();
  // tagServices.getTag();
  // tagServices.listTag();
  // tagServices.cleanUpTag();
  discordMessageServices.createDiscordMessage();
  discordMessageServices.deleteDiscordMessage();
  discordMessageServices.getDiscordMessage();
  discordMessageServices.listDiscordMessage();
  discordMessageServices.cleanUpDiscordMessage();
  // discordServices.createDiscordThread();
  // discordServices.deleteDiscordThread();
  // discordServices.getDiscordThread();
  // discordServices.deleteDiscordGuild();
  // discordServices.listDiscordGuild();
};

export default main;
