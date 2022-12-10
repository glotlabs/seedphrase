import init from "../wasm/seedphrase";
import { homePage } from "../wasm/seedphrase";

const wasmInitPromise = init("/wasm/seedphrase_bg.wasm");
let page = null;

self.addEventListener("message", (e) => {
  switch (e.data.type) {
    case "init":
      initPage(e.data.args);
      break;
  }
});

async function initPage(args: any[]) {
  await wasmInitPromise;

  // @ts-ignore
  page = homePage.apply(self, args);
  console.log(page.id());
}
