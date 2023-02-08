import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createDiscordMessage = () => {
  group("Disocrd - Should create discord message", () => {
    let discordMessageId = `${Math.floor(Math.random() * 100000000)}`;

    let createDiscordMessagePayload = {
      message_id: discordMessageId,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages/create`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "createDiscordMessage - POST /discord/messages/create - response status should be 201":
          (r) => r.status === 201,
        "createDiscordMessage - POST /discord/messages/create - response body should have id":
          (r) =>
            typeof r.json().discord_message.id !== undefined &&
            r.json().discord_message.id !== null,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct message_id":
          (r) =>
            r.json().discord_message.message_id ===
            createDiscordMessagePayload.message_id.toString(),
        "createDiscordMessage - POST /discord/messages/create - response body should have correct content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct markdown content":
          (r) =>
            r.json().discord_message.markdown_content ===
            createDiscordMessagePayload.markdown_content,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct url":
          (r) =>
            r.json().discord_message.url.url ===
            createDiscordMessagePayload.url,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_message.created_timestamp_at_discord) /
              1000 ===
            createDiscordMessagePayload.created_timestamp_at_discord,
        "createDiscordMessage - POST /discord/messages/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_message.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_message.created_timestamp_at_curioucity !== null,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct order_in_thread":
          (r) =>
            r.json().discord_message.order_in_thread ===
            createDiscordMessagePayload.order_in_thread,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/messages/${discordMessageId}`,
        undefined,
        { headers }
      ),
      {
        "createDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const deleteDiscordMessage = () => {
  group("DiscordMessage - Should delete disocrd message", () => {
    const messageId = `${Math.floor(Math.random() * 100000000)}`;

    let createDiscordMessagePayload = {
      message_id: messageId,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages/create`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "deleteDiscordMessage - POST /discord/messages/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/messages/${messageId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const getDiscordMessage = () => {
  group("DiscordMessage - Should get discord message", () => {
    // Should return not found when try to get not exist discord message
    const notExistMessageId = `${Math.floor(Math.random() * 100000000)}`;

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/messages/${notExistMessageId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordMessage - GET /discord/messages/:message_id - not exist discord message, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created discord message

    const newMessageId = `${Math.floor(Math.random() * 100000000)}`;

    let createDiscordMessagePayload = {
      message_id: newMessageId,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages/create`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "getDiscordMessage - POST /discord/messages/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/messages/${newMessageId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordMessage - GET /discord/messages/:message_id - response status should be 200":
          (r) => r.status === 200,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have id":
          (r) =>
            typeof r.json().discord_message.id !== undefined &&
            r.json().discord_message.id !== null,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct message_id":
          (r) =>
            r.json().discord_message.message_id ===
            createDiscordMessagePayload.message_id.toString(),
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct markdown content":
          (r) =>
            r.json().discord_message.markdown_content ===
            createDiscordMessagePayload.markdown_content,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct url":
          (r) =>
            r.json().discord_message.url.url ===
            createDiscordMessagePayload.url,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_message.created_timestamp_at_discord) /
              1000 ===
            createDiscordMessagePayload.created_timestamp_at_discord,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_message.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_message.created_timestamp_at_curioucity !== null,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct order_in_thread":
          (r) =>
            r.json().discord_message.order_in_thread ===
            createDiscordMessagePayload.order_in_thread,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/messages/${newMessageId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};
