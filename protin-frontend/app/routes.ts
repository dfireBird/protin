import { type RouteConfig, index, route } from "@react-router/dev/routes";

export default [
  index("routes/editor.tsx"),
  route(":pasteId", "routes/paste.tsx"),
] satisfies RouteConfig;
