import FHIR from "fhirclient";
import Client from "fhirclient/lib/Client";
import { FetchFhirClient } from "@bonfhir/core/r4b";

let smartFlowClientPromise: Promise<Client> | null = null;

export const getSmartFlowClientPromise = () => {
  if (!smartFlowClientPromise) {
    smartFlowClientPromise = FHIR.oauth2.ready();
  }

  return smartFlowClientPromise;
};

const BASE_URL_PLACEHOLDER = "{{BASE_URL_PLACEHOLDER}}";

export const smartOnFhirClient = new FetchFhirClient({
  baseUrl: BASE_URL_PLACEHOLDER,
  fetch: async (input, init) => {
    const smartFlowClient = await getSmartFlowClientPromise();

    await smartFlowClient.refreshIfNeeded();
    if (typeof input !== "string") {
      throw new Error("Only strings are handled as urls in this fetch wrapper");
    }

    const newInput = input.replace(
      BASE_URL_PLACEHOLDER,
      smartFlowClient.state.serverUrl,
    );

    const authorizationHeader = smartFlowClient.getAuthorizationHeader();
    if (!authorizationHeader) {
      throw new Error(
        "Unable to get authorization header from SMART on FHIR client",
      );
    }

    const response = await fetch(newInput, {
      ...init,
      headers: {
        ...(init?.headers ?? {}),
        Authorization: authorizationHeader,
      },
    });

    return response;
  },
  onError: console.error,
});
