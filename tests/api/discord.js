import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createDiscordMessage = () => {
  group("Tag - Should create discord message", () => {
    let createDiscordMessagePayload = {
      message_id: 1234567,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: "https://discord.com/id/4384",
      created_timestamp_at_discord: "2014-11-28T12:45:59",
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags`,
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
            typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
        "createDiscordMessage - POST /discord/messages - response body should have correct message_id":
          (r) =>
            r.json().discord_message.message_id ===
            createDiscordMessagePayload.message_id,
        "createDiscordMessage - POST /discord/messages - response body should have correct content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "createDiscordMessage - POST /discord/messages - response body should have correct markdown content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "createDiscordMessage - POST /discord/messages - response body should have correct url":
          (r) =>
            r.json().discord_message.url === createDiscordMessagePayload.url,
        "createDiscordMessage - POST /discord/messages - response body should have correct created_timestamp_at_discord":
          (r) =>
            r.json().discord_message.created_timestamp_at_discord ===
            createDiscordMessagePayload.created_timestamp_at_discord,
        "createDiscordMessage - POST /discord/messages - response body should have created_timestamp_at_curioucity":
          (r) =>
            r.json().discord_message.created_timestamp_at_curioucity &&
            r.json().discord_message.created_timestamp_at_curioucity !==
              createDiscordMessagePayload.created_timestamp_at_discord,
      }
    );

    let deleteTagPayload = {
      name: tagName,
    };

    check(
      http.request(
        "DELETE",
        `${API_HOST}/tags`,
        JSON.stringify(deleteTagPayload),
        { headers }
      ),
      {
        "createTag - DELETE /tags - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};
