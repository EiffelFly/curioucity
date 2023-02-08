import * as urlServices from "./url.js";
import * as tagServices from "./tag.js";
import * as discordServices from "./discord.js";

export const options = {};

export const API_HOST = "http://localhost:8080";

const main = () => {
  urlServices.createUrl();
  urlServices.deleteUrl();
  urlServices.getUrl();
  urlServices.listUrl();
  tagServices.createTag();
  tagServices.deleteTag();
  tagServices.getTag();
  tagServices.listTag();
  discordServices.createDiscordMessage();
  discordServices.deleteDiscordMessage();
  discordServices.getDiscordMessage();
  discordServices.listDiscordMessage();
};

export default main;
