# `Intervention` Table

The `intervention` table records all service operations performed on vehicles, such as maintenance or repair.
Each intervention is associated with a specific car and client, and includes information like date, mileage, pricing, and service details.

## Fields

| Field               | Type                                                                                                                    | Description                                                                    |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `car_id`            | `record<car>`                                                                                                           | Reference to the car involved in the intervention.                             |
| `client`            | `record<client>`                                                                                                        | Reference to the client who requested or authorized the intervention.          |
| `intervention_date` | `datetime`                                                                                                              | The actual date the intervention took place.                                   |
| `intervention_type` | `FLEXIBLE`:<br>• `{ Maintenance: { filter_air, filter_cabin, filter_gasoil, filter_oil, spark_plug } }`<br>• `'Repair'` | Type of intervention: a structured maintenance operation or a general repair.  |
| `mileage`           | `decimal`                                                                                                               | Mileage of the car at the time of intervention.                                |
| `price`             | `decimal`                                                                                                               | Total cost of the intervention.                                                |
| `remark`            | `array<string>`                                                                                                         | Additional notes or comments. Each entry is a free-text remark.                |
| `created_at`        | `datetime`                                                                                                              | Timestamp when the intervention record was created. Defaults to `time::now()`. |
| `updated_at`        | `datetime`                                                                                                              | Timestamp when the record was last updated. Defaults to `time::now()`.         |


### intervertion_type: Maintenance Structure

```
{
  "Maintenance": {
    "filter_air": true | false,
    "filter_cabin": true | false,
    "filter_gasoil": true | false,
    "filter_oil": true | false,
    "spark_plug": true | false
  }
}
```
This allows precise tracking of which components were replaced or serviced.

## Events

No events are defined for this table.

## Indexes

No indexes are currently defined for the `intervention` table.
Consider indexing `intervention_date`, `client`, or `car_id` for performance improvements.

## Relationships

- **Belongs to Car**: Each intervention links to a car via the `car_id` field.
- **Belongs to Client**: Each intervention is also tied to the `client` who owns the vehicle or authorized the service.

## Notes
- All fields have full permissions, meaning unrestricted read/write access.
- Both `created_at` and updated_at fields are managed automatically.
- The flexible `intervention_type` design allows structured maintenance while still supporting generic repair records.
- The `remark` field supports multiple textual notes, useful for storing mechanics' observations or customer comments.
