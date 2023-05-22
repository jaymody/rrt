// wasm-bindgen-rayon (which let's our wasm code us multiple threads) requires
// the use of SharedArrayBuffer, which requires cross origin isolation policies,
// see: https://github.com/GoogleChromeLabs/wasm-bindgen-rayon#setting-up
module.exports = function (app) {
  app.use((req, res, next) => {
    res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
    res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
    next()
  })
}
