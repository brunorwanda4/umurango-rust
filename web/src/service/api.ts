function normalizeApiBaseUrl(raw: string): string {
  const trimmed = raw.trim().replace(/\/+$/, "");
  if (!trimmed) {
    return "https://umurango-rust.onrender.com/api/v1";
  }
  return trimmed.endsWith("/api/v1") ? trimmed : `${trimmed}/api/v1`;
}

const apiBaseUrl = normalizeApiBaseUrl(
  process.env.NEXT_PUBLIC_API_URL ?? "https://umurango-rust.onrender.com/api/v1",
);

export type Family = {
  id: string;
  name: string;
  currency: string;
};

export type User = {
  id: string;
  full_name: string;
  phone_number?: string | null;
  email?: string | null;
  avatar_url?: string | null;
  status: string;
};

export const DEFAULT_AVATAR_STYLE_BASE_URL = "https://api.dicebear.com/10.x/weave/svg";

export function generateDefaultAvatarUrl(seed: string): string {
  const clean = seed.trim();
  if (!clean) {
    return DEFAULT_AVATAR_STYLE_BASE_URL;
  }
  return `${DEFAULT_AVATAR_STYLE_BASE_URL}?seed=${encodeURIComponent(clean)}`;
}

export async function listFamilies(): Promise<Family[]> {
  const response = await fetch(`${apiBaseUrl}/families`, {
    cache: "no-store",
  });

  if (!response.ok) {
    throw new Error(`Failed to load families: ${response.status}`);
  }

  return response.json();
}
