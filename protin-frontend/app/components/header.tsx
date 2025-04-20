import { useToolbar } from "~/providers/toolbar";
import { Button } from "./button";

export function Header() {
  const { toolbarState } = useToolbar();
  let buttonText = "";
  if (toolbarState.state === "save") {
    buttonText = "Save";
  } else if (toolbarState.state === "new") {
    buttonText = "New";
  } else {
    buttonText = "Loading";
  }

  const isLoading = toolbarState?.state === "loading";
  const onClickHandler =
    toolbarState.state !== "loading" ? toolbarState.action : undefined;

  return (
    <nav className="flex items-center justify-between px-2 pb-2 pt-1 text-xl md:px-6 lg:px-8 2xl:px-10 border-b-1 border-border/10">
      <div>
        <h1 className="font-medium text-foreground">Protin</h1>
      </div>
      <div>
        <Button
          disabled={isLoading}
          onClick={onClickHandler}
          suppressHydrationWarning // suppress warning related to using context to change the text and disabled
        >
          {buttonText}
        </Button>
      </div>
    </nav>
  );
}
