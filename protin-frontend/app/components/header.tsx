import { useToolbar } from "~/providers/toolbar";
import { Button } from "./button";
import { Loader } from "./loader";

const Copy = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="1em"
    height="1.3em"
    viewBox="0 0 24 24"
  >
    <g
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
    >
      <rect width={14} height={14} x={8} y={8} rx={2} ry={2}></rect>
      <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>
    </g>
  </svg>
);

const CopyCheck = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="1em"
    height="1.3em"
    viewBox="0 0 24 24"
  >
    <g
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
    >
      <path d="m12 15l2 2l4-4"></path>
      <rect width={14} height={14} x={8} y={8} rx={2} ry={2}></rect>
      <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>
    </g>
  </svg>
);

export function Header() {
  const { toolbarState } = useToolbar();

  let buttonChild: React.ReactNode;
  if (toolbarState.state === "save") {
    buttonChild = "Save";
  } else if (toolbarState.state === "new") {
    buttonChild = "New";
  } else {
    buttonChild = <Loader />;
  }

  const isLoading = toolbarState?.state === "loading";
  const isError = toolbarState?.state === "error";
  const onClickHandler =
    toolbarState.state === "loading" || toolbarState.state === "error"
      ? undefined
      : toolbarState.action;

  const isCopySuccess = toolbarState.copyState === "success";

  return (
    <nav className="flex items-center justify-between p-2 px-10 border-b border-border/40">
      <div>
        <h1 className="text-3xl font-medium text-foreground">Protin</h1>
      </div>
      <div className="flex min-w-fit items-center gap-3">
        {toolbarState?.copyAction && (
          <Button
            buttonColor={isCopySuccess ? "accent" : "secondary"}
            onClick={toolbarState.copyAction}
          >
            {isCopySuccess ? <CopyCheck /> : <Copy />}
          </Button>
        )}
        {!isError && (
          <Button
            disabled={isLoading}
            onClick={onClickHandler}
            suppressHydrationWarning // suppress warning related to using context to change the text and disabled
          >
            {buttonChild}
          </Button>
        )}
      </div>
    </nav>
  );
}
