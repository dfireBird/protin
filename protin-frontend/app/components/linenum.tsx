import React from "react";

const NEWLINE_REGEX = /\n/g;

export function LineNum({ content }: { content: string }) {
  const numLines = (content.match(NEWLINE_REGEX)?.length ?? 0) + 1;
  return (
    <>
      {Array.from({ length: numLines }).map((_, idx) => (
        <React.Fragment key={`line-${idx + 1}`}>
          <span>{idx + 1}</span>
          <br />
        </React.Fragment>
      ))}
    </>
  );
}
