import { useToolbar } from "~/providers/toolbar";
import { Button } from "./button";
import { Loader } from "./loader";

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

  return (
    <nav className="flex items-center justify-between p-2 px-10 border-b-1 border-border/40">
      <div>
        <h1 className="text-3xl font-medium text-foreground">Protin</h1>
      </div>
      <div>
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
