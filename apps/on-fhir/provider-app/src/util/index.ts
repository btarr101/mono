import type { Patient } from "@bonfhir/core/r4b";

export const extractPatientName = (patient: Patient) =>
  patient.name?.find((name) => name.use === "official") ??
  patient.name?.toSorted(
    (a, b) => Number(b.period?.end ?? 0) - Number(a.period?.end ?? 0),
  )?.[0];

export const calculateAge = (birthDate: string) => {
  const birth = new Date(birthDate);
  const now = new Date();

  let age = now.getFullYear() - birth.getFullYear();
  const monthDiff = now.getMonth() - birth.getMonth();
  const dayDiff = now.getDate() - birth.getDate();

  if (monthDiff < 0 || (monthDiff === 0 && dayDiff < 0)) {
    age--;
  }

  return age;
};
