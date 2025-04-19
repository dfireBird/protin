import { NavLink } from "react-router";

import { Button } from "./button";

type PageType = "new" | "paste";

export function NavBar({ pageType }: { pageType: PageType }) {
  return (
    <nav className="flex items-center justify-between px-2 pb-2 pt-1 text-xl md:px-6 lg:px-8 2xl:px-10 border-b-1 border-foreground/10">
      <div>
        <NavLink to={"/"}>
          <h1 className="font-medium text-foreground">Protin</h1>
        </NavLink>
      </div>
      <div>
        <Button>{pageType === "new" ? "Save" : "New"}</Button>
      </div>
    </nav>
  );
}
