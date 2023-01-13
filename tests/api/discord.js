import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createDiscordMessage = () => {
  group("Tag - Should create discord message", () => {
    let createDiscordMessagePayload = {
      message_id: 4884318411132,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: "https://discord.com/id/39321331",
      created_timestamp_at_discord: 167232982,
      order_in_thread: 20,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "createDiscordMessage - POST /discord/messages - response status should be 201":
          (r) => r.status === 201,
        "createDiscordMessage - POST /discord/messages - response body should have id":
          (r) =>
            typeof r.json().discord_message.id !== undefined ||
            r.json().discord_message.id !== null,
        // "createDiscordMessage - POST /discord/messages - response body should have correct message_id":
        //   (r) =>
        //     r.json().discord_message.message_id ===
        //     createDiscordMessagePayload.message_id,
        "createDiscordMessage - POST /discord/messages - response body should have correct content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "createDiscordMessage - POST /discord/messages - response body should have correct markdown content":
          (r) =>
            r.json().discord_message.markdown_content ===
            createDiscordMessagePayload.markdown_content,
        "createDiscordMessage - POST /discord/messages - response body should have correct url":
          (r) =>
            r.json().discord_message.url.url ===
            createDiscordMessagePayload.url,
        // "createDiscordMessage - POST /discord/messages - response body should have correct created_timestamp_at_discord":
        //   (r) =>
        //     r.json().discord_message.created_timestamp_at_discord ===
        //     createDiscordMessagePayload.created_timestamp_at_discord,
        // "createDiscordMessage - POST /discord/messages - response body should have created_timestamp_at_curioucity":
        //   (r) => r.json().discord_message.created_timestamp_at_curioucity,
        // "createDiscordMessage - POST /discord/messages - response body should have correct order_in_thread":
        //   (r) =>
        //     r.json().discord_message.order_in_thread ===
        //     createDiscordMessagePayload.order_in_thread,
      }
    );
  });
};
