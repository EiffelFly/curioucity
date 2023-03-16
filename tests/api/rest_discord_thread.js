import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createDiscordThread = () => {
  group("Disocrd - Should create discord thread", () => {
    const discordThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordThreadPayload = {
      thread_id: discordThreadId,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/threads/create`,
        JSON.stringify(createDiscordThreadPayload),
        {
          headers,
        }
      ),
      {
        "createDiscordThread - POST /discord/threads/create - response status should be 201":
          (r) => r.status === 201,
        "createDiscordThread - POST /discord/threads/create - response body should have id":
          (r) =>
            typeof r.json().discord_thread.id !== undefined &&
            r.json().discord_thread.id !== null,
        "createDiscordThread - POST /discord/threads/create - response body should have correct thread_id":
          (r) =>
            r.json().discord_thread.thread_id ===
            createDiscordThreadPayload.thread_id.toString(),
        "createDiscordThread - POST /discord/threads/create - response body should have correct markdown content":
          (r) =>
            r.json().discord_thread.markdown_content ===
            createDiscordThreadPayload.markdown_content,
        "createDiscordThread - POST /discord/threads/create - response body should have correct url":
          (r) =>
            r.json().discord_thread.url.url === createDiscordThreadPayload.url,
        "createDiscordThread - POST /discord/threads/create - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_thread.created_timestamp_at_discord) /
              1000 ===
            createDiscordThreadPayload.created_timestamp_at_discord,
        "createDiscordThread - POST /discord/threads/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_thread.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_thread.created_timestamp_at_curioucity !== null,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/threads/${discordThreadId}`,
        undefined,
        { headers }
      ),
      {
        "createDiscordThread - POST /discord/threads/:thread_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const deleteDiscordThread = () => {
  group("Discord - Should delete disocrd message", () => {
    const discordThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordThreadPayload = {
      thread_id: discordThreadId,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/threads/create`,
        JSON.stringify(createDiscordThreadPayload),
        {
          headers,
        }
      ),
      {
        "deleteDiscordThread - POST /discord/threads/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/threads/${discordThreadId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordThread - POST /discord/threads/:thread_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const getDiscordThread = () => {
  group("Discord - Should get discord thread", () => {
    // Should return not found when try to get not exist discord message
    const notExistThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/threads/${notExistThreadId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordThread - GET /discord/threads/:thread_id - not exist discord thead, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created discord message

    const newThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordThreadPayload = {
      thread_id: newThreadId,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/threads/create`,
        JSON.stringify(createDiscordThreadPayload),
        {
          headers,
        }
      ),
      {
        "getDiscordThread - POST /discord/threads/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/threads/${newThreadId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordThread - GET /discord/threads/:thread_id - response status should be 200":
          (r) => r.status === 200,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have id":
          (r) =>
            typeof r.json().discord_thread.id !== undefined &&
            r.json().discord_thread.id !== null,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct thread_id":
          (r) =>
            r.json().discord_thread.thread_id ===
            createDiscordThreadPayload.thread_id.toString(),
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct markdown content":
          (r) =>
            r.json().discord_thread.markdown_content ===
            createDiscordThreadPayload.markdown_content,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct url":
          (r) =>
            r.json().discord_thread.url.url === createDiscordThreadPayload.url,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_thread.created_timestamp_at_discord) /
              1000 ===
            createDiscordThreadPayload.created_timestamp_at_discord,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_thread.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_thread.created_timestamp_at_curioucity !== null,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/threads/${newThreadId}`,
        undefined,
        { headers }
      ),
      {
        "getDiscordThread - DELETE /discord/threads/:thread_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const listDiscordThread = () => {
  group("Discord - Should list discord threads", () => {
    const testSize = 10;
    const newDiscordThreads = [];

    for (let i = 0; i < testSize; i++) {
      const createDiscordThreadPayload = {
        thread_id: `${Math.floor(Math.random() * 100000000)}`,
        markdown_content: "Hi i am here",
        url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
        created_timestamp_at_discord: 1675220675,
      };
      newDiscordThreads.push(createDiscordThreadPayload);
    }

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request("GET", `${API_HOST}/discord/messages`, undefined, {
        headers,
      }),
      {
        "listDiscordThread - GET /discord/threads - should response 200": (r) =>
          r.status === 200,
        // In the future, we have to stop protobuf from emit default value
        "listDiscordThread - GET /discord/threads - no discord thread exist": (
          r
        ) => r.json().discord_messages === undefined,
      }
    );

    for (const discordThread of newDiscordThreads) {
      check(
        http.request(
          "POST",
          `${API_HOST}/discord/threads/create`,
          JSON.stringify(discordThread),
          {
            headers,
          }
        ),
        {
          "listDiscordThread - POST /discord/threads/create - response status should be 201":
            (r) => r.status === 201,
        }
      );
    }

    check(
      http.request("GET", `${API_HOST}/discord/threads`, undefined, {
        headers,
      }),
      {
        "listDiscordThread - GET /discord/threads - should response 200": (r) =>
          r.status === 200,
        "listDiscordThread - GET /discord/threads - should have 10 discord messages":
          (r) => r.json().discord_threads.length === 10,
      }
    );

    for (const discordThread of newDiscordThreads) {
      check(
        http.request(
          "DELETE",
          `${API_HOST}/discord/threads/${discordThread.thread_id}`,
          undefined,
          {
            headers,
          }
        ),
        {
          "listDiscordThread - DELETE /discord/threads/:thread_id - response status should be 204":
            (r) => r.status === 204,
        }
      );
    }
  });
};
