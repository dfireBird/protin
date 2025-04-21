import { useEffect } from "react";
import { useNavigate } from "react-router";

import { LineNum } from "~/components/linenum";
import { useToolbar } from "~/providers/toolbar";

import type { Route } from "./+types/paste";

export async function loader({
  params: { pasteId },
  request,
}: Route.LoaderArgs) {
  const origin = new URL(request.url).origin;
  const resp = await fetch(`${origin}/api/paste/${pasteId}`);
  if (resp.ok) {
    const content = await resp.text();
    return { content };
  }
  {
    // fixme: handle errors from resp
    return { error: "Error" };
  }
}

export default function Paste({ loaderData }: Route.ComponentProps) {
  if (loaderData.error) {
    // handle errors
    return;
  }

  const navigate = useNavigate();
  const { setToolbarState } = useToolbar();

  const onClickHandler = () => navigate("/");

  useEffect(() => {
    setToolbarState({ state: "new", action: onClickHandler });
  }, []);

  const content = loaderData.content;
  return (
    <div className="h-full flex">
      <div className="line h-full p-2 pb-0 border-r-1 border-border/10 text-md font-medium">
        <LineNum content={content ?? ""} />
      </div>
      <pre className="font-mono h-full p-2 pb-0 flex-auto text-md font-medium">
        <code>{content}</code>
      </pre>
    </div>
  );
}
