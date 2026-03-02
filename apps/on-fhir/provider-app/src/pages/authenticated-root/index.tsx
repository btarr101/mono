import { FhirQueryProvider } from "@bonfhir/query/r4b";
import { Outlet } from "react-router";

import { smartOnFhirClient } from "../../smart-on-fhir-client";

export const AuthenticatedRoot = () => (
  <FhirQueryProvider fhirClient={smartOnFhirClient}>
    <Outlet />
  </FhirQueryProvider>
);
