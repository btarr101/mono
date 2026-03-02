import { useFhirSearch } from "@bonfhir/query/r4b";
import { useState } from "react";
import { useForm } from "react-hook-form";

import { Button } from "@/components/ui/button";
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Spinner } from "@/components/ui/spinner";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

type PatientSearchForm = {
  firstName: string;
  lastName: string;
};

export const Patients = () => {
  const patientSearchForm = useForm<PatientSearchForm>();
  const [patientSearch, setPatientSearch] = useState<PatientSearchForm | null>(
    null,
  );

  const patients = useFhirSearch(
    "Patient",
    (search) =>
      search.given(patientSearch?.firstName).family(patientSearch?.lastName),
    undefined,
    {
      query: {
        enabled: patientSearchForm.formState.isSubmitSuccessful,
      },
    },
  );

  const searchButtonDisabled =
    patients.isLoading || !patientSearchForm.formState.isDirty;

  return (
    <div className="m-4 space-y-4 flex flex-col">
      <h3>Patients</h3>

      <form
        onSubmit={patientSearchForm.handleSubmit((values) => {
          setPatientSearch(values);
          patientSearchForm.reset(values);
        })}
      >
        <FieldGroup className="flex flex-row items-end">
          <Field className="w-fit">
            <FieldLabel>First Name</FieldLabel>
            <Input {...patientSearchForm.register("firstName")} />
          </Field>
          <Field className="w-fit">
            <FieldLabel>Last Name</FieldLabel>
            <Input {...patientSearchForm.register("lastName")} />
          </Field>
          <Button
            className="w-fit h-fit"
            disabled={searchButtonDisabled}
            type="submit"
          >
            {patients.isLoading ? "Searching..." : "Search"}
          </Button>
        </FieldGroup>
      </form>
      {patients.isLoading ? (
        <Spinner />
      ) : (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>DOB (Age)</TableHead>
              <TableHead>Sex</TableHead>
              <TableHead>MRN</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {patients.data?.searchMatch().map((patient) => (
              <TableRow key={patient.id}>
                <TableCell>{patient.name?.[0]?.text}</TableCell>
                <TableCell>{patient.birthDate} (209)</TableCell>
                <TableCell>{patient.gender}</TableCell>
                <TableCell>{patient.identifier?.[0]?.value}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
};
