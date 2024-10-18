import { PublicKey } from "@solana/web3.js";
import { evmAddressToPaddedArray, readKeypairToFile } from "./Helper";

export const LST_RESTAKING_PROGRAM_ID = new PublicKey(
  "DMKWjKA56Wk3stpGjkMJ6YYDS58TvowZEShdg3AYiH17"
);

export const ENDPOINT_PROGRAM_ID = new PublicKey(
  "76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6"
);
export const ULN302_PROGRAM_ID = new PublicKey(
  "7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH"
);

export const EXECUTOR_PROGRAM_ID = new PublicKey(
  "6doghB248px58JSSwG4qejQ46kFMW4AMj7vzJnWZHNZn"
);

export const PRICE_FEED_PROGRAM_ID = new PublicKey(
  "8ahPGPjEbpgGaZx2NV1iG5Shj7TDwvsjkEDcGWjt94TP"
);

export const DVN_PROGRAM_ID = new PublicKey(
  "HtEYV4xB4wvsj5fgTkcfuChYpvGYzgzwvNhgDZQNh7wW"
);

export const SYSTEM_PROGRAM_ID = new PublicKey(
  "11111111111111111111111111111111"
);

export const remoteOapp = evmAddressToPaddedArray(
    "0x8Db51d2E55453935b790e23529de314143D2160a"
  // "0xEAf4E4D09b9CeB936492518A852026c914beb11E"
);

export const remotePeer = "0x8Db51d2E55453935b790e23529de314143D2160a";
export const remoteEid = 40259;

export const SEND_LIBRARY_INFO = new PublicKey(
  "526PeNZfw8kSnDU4nmzJFVJzJWNhwmZykEyJr5XWz5Fv"
);
export const ENDPOINT_SETTINGS = new PublicKey(
  "2uk9pQh3tB5ErV7LGQJcbWjb4KeJ2UJki5qJZ8QG56G3"
);
export const ULN_SETTINGS = new PublicKey(
  "2XgGZG4oP29U3w5h4nTk1V2LFHL23zKDPJjs3psGzLKQ"
);
export const EXECUTOR_CONFIG = new PublicKey(
  "AwrbHeCyniXaQhiJZkLhgWdUCteeWSGaSN1sTfLiY7xK"
);
export const PRICE_FEED_CONFIG = new PublicKey(
  "CSFsUupvJEQQd1F4SsXGACJaxQX4eropQMkGV2696eeQ"
);
export const DVN_CONFIG = new PublicKey(
  "4VDjp6XQaxoZf5RGwiPU9NR1EXSZn2TP4ATMmiSzLfhb"
);

export const ENDPOINT_EVENT_AUTHORITY = new PublicKey(
  "F8E8QGhKmHEx2esh5LpVizzcP4cHYhzXdXTwg9w3YYY2"
);
export const ULN_EVENT_AUTHORITY = new PublicKey(
  "7n1YeBMVEUCJ4DscKAcpVQd6KXU7VpcEcc15ZuMcL4U3"
);

export const testKeys = async () => {
  const OWNER = await readKeypairToFile("owner");
  const NEW_OWNER = await readKeypairToFile("new_owner");
  const USER = await readKeypairToFile("user");
  const INVALID_OWNER = await readKeypairToFile("user2");
  const ANOTHER_USER = await readKeypairToFile("user2");
  const DELEGATE = await readKeypairToFile("delegate");
  const DEV = await readKeypairToFile("dev");

  return [OWNER, NEW_OWNER, USER, INVALID_OWNER, ANOTHER_USER, DELEGATE, DEV];
};
