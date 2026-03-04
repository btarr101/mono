import type { DefaultFormatter } from "@bonfhir/core/r4b";
import { useFhirFormatter } from "@bonfhir/react/r4b";

export const useDefaultFhirFormatter = () => {
  const { formatter, ...rest } = useFhirFormatter();

  return {
    formatter: formatter as DefaultFormatter,
    ...rest,
  };
};
