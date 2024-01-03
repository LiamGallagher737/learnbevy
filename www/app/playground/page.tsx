export const runtime = "edge";

import { DEFAULT_CODE } from "@/lib/constants";
import { notFound } from "next/navigation";
import ClientPlayground from "./client-playground";

export default async function Page({
  searchParams,
}: {
  searchParams: { [key: string]: string | undefined };
}) {
  let code = DEFAULT_CODE;

  const shareId = searchParams["share"];
  if (shareId) {
    const result = await process.env.SHARES.get(shareId);
    if (result) {
      code = result;
    } else {
      return notFound();
    }
  }

  return (
    <main className="p-4 h-screen">
      <ClientPlayground code={code} />
    </main>
  );
}
