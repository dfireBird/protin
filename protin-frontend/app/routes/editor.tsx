import { useEffect, useRef, useState } from "react";

import type React from "react";
import type { Route } from "./+types/editor";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Protin" },
    { name: "description", content: "Protin - Beefed up Text Storage Site!" },
  ];
}

export default function Editor() {
  const textArea = useRef<HTMLTextAreaElement>(null);
  const [content, setContent] = useState("");
  const [selectionStart, setSelectionStart] = useState(0);

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
        {">"}
      </div>
      <textarea
        className="font-mono h-full p-2 pb-0 flex-auto text-md font-medium focus-visible:outline-none"
        ref={textArea}
        value={content}
        onKeyDown={insertSpaceOnTab}
        onChange={onChange}
      ></textarea>
    </div>
  );
}
