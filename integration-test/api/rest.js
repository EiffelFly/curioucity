import { createUrl } from "./createUrl.js";

export const options = {
  hosts: { address: "0.0.0.0:8010" },
};

const main = () => {
  createUrl();
};

export default main;
