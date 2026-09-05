import { useEffect } from "react";
import { data, useNavigate } from "react-router";

import { LineNum } from "~/components/linenum";
import { useToolbar } from "~/providers/toolbar";

import type { Route } from "./+types/paste";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Protin" },
    { name: "description", content: "Protin - Beefed up Text Storage Site!" },
  ];
}

export async function clientLoader({
  params: { pasteId },
  request,
}: Route.LoaderArgs) {
  const origin = new URL(request.url).origin;
  const resp = await fetch(`${origin}/api/paste/${pasteId}`);
  if (resp.ok) {
    const content = await resp.text();
    return { content };
  } else {
    throw data({
      error: await resp.text(),
    }, {
      status: 404,
      statusText: "Request Paste is not found"
    })
  }
}

export default function Paste({ loaderData }: Route.ComponentProps) {
  const navigate = useNavigate();
  const { setToolbarState } = useToolbar();

  const onClickHandler = () => {
    setToolbarState({ state: "loading" });
    navigate("/");
  };

  useEffect(() => {
    setToolbarState({ state: "new", action: onClickHandler });
  }, []);

  const content = loaderData.content;
  return (
    <main className="flex-auto flex font-mono">
      <div className="line min-w-11 p-2 border-r border-border/40 text-md font-medium">
        <LineNum content={content} />
      </div>
      <pre className="font-mono h-full p-2 pb-0 flex-auto text-md font-medium">
        <code>{content}</code>
      </pre>
    </main>
  );
}
