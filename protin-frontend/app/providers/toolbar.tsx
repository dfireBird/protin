import { createContext, useContext, useState } from "react";

type OnClickHandler = React.MouseEventHandler<HTMLButtonElement>;
type SetToolbarState = React.Dispatch<React.SetStateAction<ToolbarState>>;

type ToolbarState =
  | { state: "loading" }
  | { state: "new"; action: OnClickHandler }
  | { state: "save"; action: OnClickHandler };

type TToolbarContext = {
  toolbarState: ToolbarState;
  setToolbarState: SetToolbarState;
};

const LOADING_STATE = { state: "loading" } as const;

const ToolbarContext = createContext<TToolbarContext | null>(null);

export function useToolbar() {
  const toolbarContext = useContext(ToolbarContext);

  if (toolbarContext === null) {
    throw new Error("useToolbar should be used in a toolbar context.");
  }

  return toolbarContext;
}

export function ToolbarProvider({ children }: { children: React.ReactNode }) {
  const [toolbarState, setToolbarState] = useState<ToolbarState>(LOADING_STATE);

  return (
    <ToolbarContext.Provider value={{ toolbarState, setToolbarState }}>
      {children}
    </ToolbarContext.Provider>
  );
}
