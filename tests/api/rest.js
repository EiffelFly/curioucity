import * as urlServices from "./url.js";
import * as tagServices from "./tag.js";
import * as discordServices from "./discord.js";

export const options = {};

export const API_HOST = "http://localhost:8080";

const main = () => {
  urlServices.createUrl();
  urlServices.deleteUrl();
  urlServices.getUrl();
  tagServices.createTag();
  tagServices.deleteTag();
  tagServices.getTag();
  tagServices.listTag();
  urlServices.listUrl();
  discordServices.createDiscordMessage();
};

export default main;
