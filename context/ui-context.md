# UI Context

## User Experience

The interface should make shared financial state easy to understand without
turning the family into an accounting department. Lead with the amount,
decision, status, or next action the member came to see.

Use calm layouts, clear hierarchy, and meaningful empty, loading, offline, and
error states. The native app must make local availability and pending sync
state understandable without alarming users.

## Surfaces

- Dashboard: family balance summaries, recent activity, pending sync, and
  communal projects.
- Money: contributions, expenses, payments, categories, and transaction detail.
- Projects: shared goals, required contributions, progress, and decisions.
- Family: members, roles, permissions, and family settings.

The web portal and native app may differ in navigation, but they must use the
same domain terms and backend contracts.

## Interaction

Actions should be short and direct: `Add expense`, `Record payment`, `Sync`,
`View details`. Confirm destructive or financially consequential actions with
the exact consequence, not generic reassurance.

Never hide pending, failed, or conflicted sync mutations behind a blank state.
