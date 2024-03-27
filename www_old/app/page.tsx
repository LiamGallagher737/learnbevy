export const runtime = "edge";

import Link from "next/link";

export default function Home() {
  return (
    <main className="flex justify-center items-center flex-col h-screen">
      <h1 className="text-xl">Coming soon...</h1>
      <p>
        In the meantime you can
        {" "}
        <Link className="underline" href="/playground">
          go to the playground
        </Link>
      </p>
    </main>
  );
}
