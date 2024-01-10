export const runtime = "edge";

import { DEFAULT_CODE } from "@/lib/constants";
import { notFound } from "next/navigation";
import ClientPlayground from "./client-playground";
import type { Metadata } from "next";
import { kv } from "@vercel/kv";
import { DEFAULT_VERSION, Version } from "@/lib/versions";
import { Channel, DEFAULT_CHANNEL } from "@/lib/channels";

export const metadata: Metadata = {
  title: "Bevy Playground",
  description: "Experiment with Bevy apps in your browser",
};

export default async function Page({
  searchParams,
}: {
  searchParams: { [key: string]: string | undefined };
}) {
  let opts: { code: string; version: Version; channel: Channel } = {
    code: DEFAULT_CODE,
    version: DEFAULT_VERSION,
    channel: DEFAULT_CHANNEL,
  };

  const shareId = searchParams["share"];
  if (shareId) {
    const result = await kv.get<{
      code: string;
      version: Version;
      channel: Channel;
    }>(shareId);

    if (result) {
      opts = result;
    } else {
      return notFound();
    }
  }

  return (
    <main className="p-4 h-screen">
      <ClientPlayground {...opts} />
    </main>
  );
}
