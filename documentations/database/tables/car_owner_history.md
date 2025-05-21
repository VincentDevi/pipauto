# `car_owner_history` Table

The `car_owner_history` table tracks ownership history of vehicles.
Each record represents a historical relationship between a car and a client, preserving past ownership even if the car.client_id changes later.
This enables a full audit trail of who has owned or been associated with each vehicle over time.

## Fields

| Field        | Type             | Description                                                                  |
| ------------ | ---------------- | ---------------------------------------------------------------------------- |
| `car`        | `record<car>`    | Reference to the car involved in the ownership history.                      |
| `client`     | `record<client>` | Reference to the client who owned or was responsible for the vehicle.        |
| `created_at` | `datetime`       | Timestamp when this ownership record was created. Defaults to `time::now()`. |


## Events

No events are defined for this table.

## Indexes

No indexes are currently defined on the `car_owner_history` table.
Indexing `car` or `client` may be useful for quick access to historical ownership data.

## Relationship
- **Car Reference**: Each record is associated with a car via the `car` field.
- **Client Reference**: Each record is associated with a client via the `client` field.
- **This table acts as a historical log** — unlike `car.client_id`, which only represents the current owner, this table preserves all past owners over time.

## Notes
- All fields are fully accessible due to full permissions.
- This table is essential for audits, resale tracking, and understanding long-term car history.
- Use this in conjunction with `car.client_id` to distinguish between current and past ownership.
