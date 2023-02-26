import hljs from "highlight.js";
import { useEffect } from "react";

type CodeBlockProps = {
    sourceCode: string
}

export default function CodeBlock({sourceCode}: CodeBlockProps) {
  useEffect(() => {
    hljs.highlightAll();
  });

  return (
    <pre>
      <code>
          {sourceCode}
      </code>
    </pre>
  );
}
