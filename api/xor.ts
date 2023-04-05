
/* @ts-ignore */
import wasm from '../wasm/pkg/wasm_bg.wasm?module'
import init, { get } from '../wasm/pkg/wasm.js';
export const config = {
  runtime: 'edge',
}




export default async function handler(request: Request, event: Event) {
    await init(wasm);

    const value = get(1);

  return new Response("Value is: " + value)
}
