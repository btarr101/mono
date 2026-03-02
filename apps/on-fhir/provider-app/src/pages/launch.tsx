import FHIR from "fhirclient";
import { useEffect } from "react";

export const Launch = () => {
  useEffect(() => {
    FHIR.oauth2.authorize({
      clientId: "49f5f7d3-829d-4bc4-bd47-ee8a5b476322",
      scope: [
        "openid",
        "offline_access",
        "profile",
        "fhirUser",
        "user/Patient.read",
      ].join(" "),
    });
  }, []);

  return <p>Loading...</p>;
};
