import type { Patient } from "@bonfhir/core/r4b";
import { useFhirSearch } from "@bonfhir/query/r4b";
import { useState } from "react";
import { useForm } from "react-hook-form";

import { calculateAge, extractPatientName } from "../../util";

type PatientSearchForm = {
  firstName: string;
  lastName: string;
  mrn: string;
};

export const Patients = () => {
  const patientSearchForm = useForm<PatientSearchForm>();
  const [patientSearchParams, setPatientSearchParams] =
    useState<PatientSearchForm | null>(null);

  const patientSearch = useFhirSearch(
    "Patient",
    (search) =>
      search
        .given(patientSearchParams?.firstName)
        .family(patientSearchParams?.lastName)
        .identifier(patientSearchParams?.mrn),
    undefined,
    {
      query: {
        enabled: patientSearchForm.formState.isSubmitSuccessful,
      },
    },
  );

  const [selectedPatient, setSelectedPatient] = useState<Patient>();

  const onValidSubmit = (values: PatientSearchForm) => {
    const allEmpty = Object.values(values).every((value) => !value);
    if (allEmpty) {
      patientSearchForm.setError("root", {
        message: "At least one search parameter is required",
      });
      return;
    }

    setSelectedPatient(undefined);
    setPatientSearchParams(() => {
      patientSearchForm.reset(values);
      return values;
    });
  };

  const searchButtonDisabled =
    patientSearch.isLoading || !patientSearchForm.formState.isDirty;

  const patients = patientSearch.data?.searchMatch();

  return (
    <div className="h-full w-full flex">
      <div className="space-y-4 flex flex-col h-full min-h-0 flex-1">
        <form onSubmit={patientSearchForm.handleSubmit(onValidSubmit)}>
          <fieldset className="fieldset space-y-2">
            <legend className="fieldset-legend text-xl">Search patients</legend>

            <div className="flex gap-4">
              <div className="space-y-2">
                <label className="label">First Name</label>
                <input
                  className="input"
                  disabled={patientSearch.isLoading}
                  type="text"
                  {...patientSearchForm.register("firstName")}
                />
              </div>
              <div className="space-y-2">
                <label className="label">Last Name</label>
                <input
                  className="input"
                  disabled={patientSearch.isLoading}
                  inputMode="numeric"
                  type="text"
                  {...patientSearchForm.register("lastName")}
                />
              </div>
              <div className="space-y-2">
                <label className="label">MRN</label>
                <input
                  className="input"
                  disabled={patientSearch.isLoading}
                  type="text"
                  {...patientSearchForm.register("mrn")}
                />
              </div>
            </div>

            <button
              className="btn w-fit"
              disabled={searchButtonDisabled}
              type="submit"
            >
              {patientSearch.isLoading ? "Searching..." : "Search"}
            </button>

            {patientSearchForm.formState.errors.root && (
              <p className="text-sm text-error">
                {patientSearchForm.formState.errors.root.message}
              </p>
            )}
          </fieldset>
        </form>
        <div className="flex-1 min-h-0 flex">
          {patientSearch.isLoading ? (
            <span className="loading loading-ball m-auto" />
          ) : patientSearch.error ? (
            <PatientSearchErrorAlert />
          ) : !patients?.length ? (
            <PatientSearchNoRecordsAlert />
          ) : (
            <PatientsTable
              patients={patients}
              selectedPatient={selectedPatient}
              setSelectedPatient={setSelectedPatient}
            />
          )}
        </div>
      </div>
      {/* TODO WORK ON PATIENT PANEL */}
      <div className="flex-1">PATIENT PANEL</div>
    </div>
  );
};

const PatientSearchErrorAlert = () => (
  <div className="alert alert-error h-min w-fit m-auto" role="alert">
    <svg
      className="h-6 w-6 shrink-0 stroke-current"
      fill="none"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
    <span>Failed to search, try narrowing your search parameters.</span>
  </div>
);

const PatientSearchNoRecordsAlert = () => (
  <div className="alert alert-warning h-min w-fit m-auto" role="alert">
    <svg
      className="h-6 w-6 shrink-0 stroke-current"
      fill="none"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
    <span>No records found, try adjusting your search parameters.</span>
  </div>
);

type PatientsTableProps = {
  patients: Patient[];
  selectedPatient?: Patient;
  setSelectedPatient: React.Dispatch<React.SetStateAction<Patient | undefined>>;
};

const PatientsTable = ({
  patients,
  selectedPatient,
  setSelectedPatient,
}: PatientsTableProps) => (
  <div className="flex-1 min-h-0 w-full border rounded-xl flex flex-col overflow-clip">
    <div className="flex-1 min-h-0 overflow-y-auto w-full">
      <table className="table table-pin-rows">
        <thead>
          <tr>
            <th>Name</th>
            <th>DOB (Age)</th>
            <th>Sex</th>
            <th>MRN</th>
          </tr>
        </thead>
        <tbody>
          {patients?.map((patient) => (
            <tr
              className={
                selectedPatient?.id === patient.id
                  ? " bg-base-300 cursor-pointer"
                  : " hover:bg-base-200 cursor-pointer"
              }
              key={patient.id}
              onClick={() =>
                setSelectedPatient((prevPatient) =>
                  prevPatient?.id === patient.id ? undefined : patient,
                )
              }
            >
              <td>{extractPatientName(patient)?.text}</td>
              <td>
                {patient.birthDate
                  ? `${patient.birthDate} (${calculateAge(patient.birthDate)})`
                  : "-"}
              </td>
              <td>{patient.gender ?? "-"}</td>
              <td>{patient.identifier?.[0]?.value ?? "-"}</td>
            </tr>
          ))}
        </tbody>
        <tfoot>
          <tr>
            <td colSpan={4}>Total: {patients?.length}</td>
          </tr>
        </tfoot>
      </table>
    </div>
  </div>
);
