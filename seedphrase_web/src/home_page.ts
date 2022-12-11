import init, { homePage } from "../wasm/seedphrase";
import { Poly } from "poly";

(async () => {
  await init("/wasm/seedphrase_bg.wasm");

  const page = homePage(location.href);
  const poly = new Poly(page);
  poly.init();
})();
