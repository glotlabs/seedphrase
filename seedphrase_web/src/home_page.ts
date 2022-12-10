import init from "../wasm/seedphrase.js";
import { homePage } from "../wasm/seedphrase";
import { Poly } from "poly";
import { defaultDebugConfig } from "poly/src/logger";

(async () => {
  await init("/wasm/seedphrase_bg.wasm");

  const page = homePage(location.href);

  const poly = new Poly(page, {
    loggerConfig: defaultDebugConfig(),
  });

  poly.init();
})();
