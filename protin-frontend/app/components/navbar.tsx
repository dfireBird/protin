import { NavLink } from "react-router";

import { Button } from "./button";

type PageType = "new" | "paste";

export function NavBar({ pageType }: { pageType: PageType }) {
  return (
    <nav className="sm:items-stretch flex items-center justify-between px-2 py-1 text-xl sm:px-6 lg:px-8 border-b-2 border-foreground/5">
      <div>
        <NavLink to={"/"}>
          <h1 className="font-medium text-foreground">Protin</h1>
        </NavLink>
      </div>
      <div>
        <Button>{pageType == "new" ? "Save" : "New"}</Button>
      </div>
    </nav>
  );
}
