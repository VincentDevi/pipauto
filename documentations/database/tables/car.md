# `Car` table

The `car` table stores detailed information about vehicles associated with clients. Each car record is linked to a specific client and includes specifications like brand, model, engine details, and metadata for service tracking.

## Fields

| Field          | Type                                    | Description                                                           |
| -------------- | --------------------------------------- | --------------------------------------------------------------------- |
| `id`           | `record<car>`                           | Unique identifier for the car.                                        |
| `brand`        | `string`                                | Brand or manufacturer of the vehicle.                                 |
| `model`        | `string`                                | Specific model name/designation.                                      |
| `year`         | `int`                                   | Manufacturing year of the car.                                        |
| `cc`           | `decimal`                               | Engine displacement in cubic centimeters.                             |
| `fuel`         | `'Gasoline'` \| `'Diesel'` \| `'Other'` | Type of fuel the vehicle uses.                                        |
| `oil_type`     | `string`                                | Type of oil recommended for this car.                                 |
| `oil_quantity` | `decimal`                               | Quantity of oil (usually in liters) required for an oil change.       |
| `client_id`    | `record<client>`                        | Reference to the client who owns this car.                            |
| `created_at`   | `datetime`                              | Timestamp when the car record was created. Defaults to `time::now()`. |
| `updated_at`   | `datetime`                              | Timestamp of the last update. Defaults to `time::now()`.              |

## Events

No events are defined for this table at this time.

## Indexes

No indexes are currently defined on the `car` table. For future optimization, consider indexing fields like `client_id`, `brand`, or `model` depending on query needs.


## Relationships

- **Belongs to Client**: Each car is associated with one client via the `client_id` field (`record<client>`).
- **Interventions (implicit)**: Though not directly referenced in this schema, cars are expected to be linked to multiple `intervention` records that document their service history.

## Notes

- All fields use full permissions, making them fully accessible for create, read, update, and delete operations.
- The `fuel` field is restricted to a union of predefined values, ensuring standardized data entry.
- The `created_at` and `updated_at` fields provide automatic tracking of record lifecycle.
