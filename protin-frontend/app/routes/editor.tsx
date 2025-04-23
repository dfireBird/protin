import { useCallback, useEffect, useRef, useState } from "react";
import { redirect, useSubmit } from "react-router";

import { useToolbar } from "~/providers/toolbar";
import { LineNum } from "~/components/linenum";

import type React from "react";
import type { Route } from "./+types/editor";

type NewPasteRespone = {
  id: string;
  // has more but no use for us
};

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Protin" },
    { name: "description", content: "Protin - Beefed up Text Storage Site!" },
  ];
}

export async function clientAction({ request }: Route.ActionArgs) {
  const origin = new URL(request.url).origin;
  const data = await request.formData();
  const resp = await fetch(`${origin}/api/paste`, {
    method: "POST",
    body: data,
  });

  if (resp.ok) {
    const data = (await resp.json()) as NewPasteRespone;
    if (data.id) {
      return redirect(`/${data.id}`);
    } else {
      // fixme: handle error in data recieved
    }
  } else {
    // fixme: handle errors
  }
  return resp.ok;
}

export default function Editor({}: Route.ComponentProps) {
  const textArea = useRef<HTMLTextAreaElement>(null);
  const [content, setContent] = useState("");
  const [selectionStart, setSelectionStart] = useState(0);

  const { setToolbarState } = useToolbar();

  const submit = useSubmit();

  const onClickToolbar = useCallback(() => {
    submit({ file: content }, { method: "POST" });
  }, [content]);

  useEffect(() => {
    setToolbarState({ state: "save", action: onClickToolbar });
  }, [onClickToolbar]);

  const insertSpaceOnTab = (
    event: React.KeyboardEvent<HTMLTextAreaElement>,
  ) => {
    if (event.key === "Tab") {
      event.preventDefault();

      const { selectionStart, selectionEnd } = event.currentTarget;
      setContent(
        (prevContent) =>
          `${prevContent.slice(0, selectionStart)}\t${prevContent.slice(selectionEnd)}`,
      );
      setSelectionStart(selectionStart + 1);
    }
  };

  const onChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setContent(event.currentTarget.value);
    setSelectionStart(event.currentTarget.selectionStart);
  };

  useEffect(() => {
    textArea.current?.setSelectionRange(selectionStart, selectionStart);
  }, [selectionStart, textArea]);

  return (
    <div className="h-full flex">
      <div className="line h-full p-2 pb-0 border-r-1 border-border/10 text-md font-medium">
        <LineNum content={content} />
      </div>
      <textarea
        className="font-mono max-h-full p-2 pb-0 flex-auto text-md font-medium focus-visible:outline-none"
        ref={textArea}
        value={content}
        onKeyDown={insertSpaceOnTab}
        onChange={onChange}
      ></textarea>
    </div>
  );
}
