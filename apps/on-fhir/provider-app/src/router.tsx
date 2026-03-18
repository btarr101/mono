import { createBrowserRouter } from "react-router";

import { Root } from "./pages";
import { AuthenticatedRoot } from "./pages/authenticated-root";
import { Patients } from "./pages/authenticated-root/patients";
import { Launch } from "./pages/launch";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: Root,
    children: [
      {
        path: "/",
        Component: AuthenticatedRoot,
        children: [
          {
            index: true,
            Component: Patients,
          },
        ],
      },
      {
        path: "/launch",
        Component: Launch,
      },
    ],
  },
]);
