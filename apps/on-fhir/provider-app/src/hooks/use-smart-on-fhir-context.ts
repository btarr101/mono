import { useFhirClient } from "@bonfhir/query/r4b";

import { getSmartFlowClientPromise } from "../smart-on-fhir-client";

export const useSmartOnFhirPatient = () =>
  useFhirClient(async (client) => {
    const smartFlowClient = await getSmartFlowClientPromise();
    const patientId = smartFlowClient.getPatientId();
    if (!patientId) {
      return null;
    }

    return await client.read("Patient", patientId);
  });
