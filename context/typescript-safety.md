# TypeScript Safety

- Keep `strict` TypeScript enabled.
- Type every API response and request boundary.
- Treat JSON from the backend as untrusted until it has the expected shape.
- Do not use `any` to bypass a typing problem. Narrow unknown values or define
  the missing contract.
- Check `response.ok` before decoding a fetch response.
- Include useful context in thrown errors without exposing credentials or
  sensitive financial details.
- Keep server-only secrets out of client components and `NEXT_PUBLIC_*`
  variables.
- Cancel or clean up browser requests, timers, subscriptions, and listeners
  when their owning component unmounts.
- Keep money as integers in client state and API types. Do not call
  `parseFloat`, use `number` arithmetic for currency, or format by rounding
  binary floating-point values.
- Run:

```powershell
cd web
bun run typecheck
bun run lint
```
