import { createUrl, getUrl } from "./url.js";

export const options = {};

export const API_HOST = "http://localhost:8080";

const main = () => {
  createUrl();
  getUrl();
};

export default main;
