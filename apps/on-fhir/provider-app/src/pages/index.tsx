import { FhirFormatterProvider } from "@bonfhir/react/r4b";
import { Outlet } from "react-router";

import { Header } from "../components/header";

export const Root = () => (
  <FhirFormatterProvider>
    <div className="flex flex-col h-screen overflow-hidden">
      <Header />
      <div className="flex-1 min-h-0 flex flex-col m-4">
        <Outlet />
      </div>
    </div>
  </FhirFormatterProvider>
);
