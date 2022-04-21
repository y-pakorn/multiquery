import { LCDClient } from "@terra-money/terra.js";
import { decodeBase64, encodeBase64 } from "./helpers";

const MULTIQUERY = "terra1swrywzkphty20e2uzpy582xu836luw0e5yp25m";
const RED_BANK = "terra19dtgj9j5j7kyf3pmejqv8vzfpxtejaypgzkz5u";
const MARS_TOKEN = "terra12hgwnpupflfpuual532wgrxu2gjp0tcagzgx4n";

const terra = new LCDClient({
  chainID: "columbus-5",
  URL: "https://lcd.terra.dev",
});

type MultiQueryResponse = {
  success: boolean;
  data: string;
}[];

(async function () {
  const query = [
    {
      bank: {
        all_balances: {
          address: RED_BANK,
        },
      },
    },
    {
      wasm: {
        smart: {
          contract_addr: MARS_TOKEN,
          msg: encodeBase64({ token_info: {} }),
        },
      },
    },
  ];

  const response: MultiQueryResponse = await terra.wasm.contractQuery(
    MULTIQUERY,
    query
  );

  const responseParsed = response.map((item) => {
    return {
      success: item.success,
      data: decodeBase64(item.data),
    };
  });

  console.log("query =", JSON.stringify(query, null, 2));
  console.log("response =", JSON.stringify(responseParsed, null, 2));
})();
