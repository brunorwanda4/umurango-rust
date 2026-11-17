import { listFamilies, type Family } from "@/service/api";

export default async function HomePage() {
  let families: Family[] = [];
  let apiUnavailable = false;

  try {
    families = await listFamilies();
  } catch {
    apiUnavailable = true;
  }

  return (
    <main className="mx-auto flex min-h-screen max-w-5xl flex-col gap-8 p-6 sm:p-10">
      <header className="flex flex-col gap-3">
        <p className="text-sm font-semibold uppercase tracking-wide text-primary">
          Umuryango
        </p>
        <h1 className="text-4xl font-bold">Family finances, governed together.</h1>
        <p className="max-w-2xl text-base-content/70">
          A local-first financial workspace for shared family decisions,
          contributions, and transparent spending.
        </p>
      </header>

      <section className="stats stats-vertical w-full shadow sm:stats-horizontal">
        <div className="stat">
          <div className="stat-title">Connected families</div>
          <div className="stat-value">{families.length}</div>
          <div className="stat-desc">From the Umuryango API</div>
        </div>
        <div className="stat">
          <div className="stat-title">Sync status</div>
          <div className="stat-value text-lg">
            {apiUnavailable ? "Offline" : "Connected"}
          </div>
          <div className="stat-desc">Local-first by design</div>
        </div>
      </section>

      <section className="card border border-base-300 bg-base-100 shadow-sm">
        <div className="card-body">
          <h2 className="card-title">Your families</h2>
          {families.length === 0 ? (
            <p className="text-base-content/70">
              No families are available yet. Start by creating one in the
              native app or API.
            </p>
          ) : (
            <div className="flex flex-col gap-3">
              {families.map((family) => (
                <div
                  className="rounded-box border border-base-300 p-4"
                  key={family.id}
                >
                  <p className="font-semibold">{family.name}</p>
                  <p className="text-sm text-base-content/70">
                    Currency: {family.currency}
                  </p>
                </div>
              ))}
            </div>
          )}
          <div className="card-actions justify-end">
            <button className="btn">Open dashboard</button>
          </div>
        </div>
      </section>
    </main>
  );
}
