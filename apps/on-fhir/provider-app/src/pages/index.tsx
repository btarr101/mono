import { Outlet } from "react-router";

import { Header } from "../components/header";

export const Root = () => (
  <div>
    <Header />
    <Outlet />
  </div>
);
