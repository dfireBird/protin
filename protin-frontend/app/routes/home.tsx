import type { Route } from "./+types/home";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Protin" },
    { name: "description", content: "Protin - Beefed up Text Storage Site!" },
  ];
}

export default function Home() {
  return <h1>Hello World</h1>;
}
