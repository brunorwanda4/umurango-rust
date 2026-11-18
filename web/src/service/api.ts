const apiBaseUrl =
  process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:4433/api/v1";

export type Family = {
  id: string;
  name: string;
  currency: string;
};

export async function listFamilies(): Promise<Family[]> {
  const response = await fetch(`${apiBaseUrl}/families`, {
    cache: "no-store",
  });

  if (!response.ok) {
    throw new Error(`Failed to load families: ${response.status}`);
  }

  return response.json();
}
