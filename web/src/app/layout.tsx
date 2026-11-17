import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Umuryango",
  description: "Family financial governance"
};

export default function RootLayout({
  children
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body className="bg-base-200 text-base-content">{children}</body>
    </html>
  );
}
