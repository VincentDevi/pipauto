# `Client` Table

The `client` table stores information about individuals or entities that own one or more vehicles and request interventions (repairs or maintenance) on them.
This is a central table in the Pipauto application, linking clients to their cars and the interventions performed on those cars.

## Schema
```
DEFINE TABLE client SCHEMAFULL PERMISSIONS
    FOR select, create, update, delete WHERE true;

DEFINE FIELD id        ON TABLE client TYPE record<client>;
DEFINE FIELD first_name ON TABLE client TYPE string;
DEFINE FIELD last_name  ON TABLE client TYPE string;
DEFINE FIELD email      ON TABLE client TYPE option<string>;
DEFINE FIELD phone      ON TABLE client TYPE string;
DEFINE FIELD address    ON TABLE client TYPE string;
DEFINE FIELD city       ON TABLE client TYPE string;
DEFINE FIELD zip_code   ON TABLE client TYPE string;

```

## Fields

| Field        | Type             | Description                              |
| ------------ | ---------------- | ---------------------------------------- |
| `id`         | `record<client>` | Unique identifier for the client record. |
| `first_name` | `string`         | The client’s first name.                 |
| `last_name`  | `string`         | The client’s last name.                  |
| `email`      | `option<string>` | Optional email address.                  |
| `phone`      | `string`         | Phone number of the client.              |
| `address`    | `string`         | Street address of the client.            |
| `city`       | `string`         | City of residence.                       |
| `zip_code`   | `string`         | ZIP or postal code.                      |

## Relationships

- **Owns Cars**: A client is linked to one or more records in the car table. Each car has a client field pointing back to its owner.
- **Indirect Interventions**: Through the cars they own, clients are also connected to intervention records. While interventions do not link directly to clients, they relate to cars, which are in turn owned by clients.

## Notes

- The `email` field is optional to support clients who do not provide one.
- All other fields are mandatory to ensure complete client contact and location information.
- Clients are central to the app's functionality and should be managed with care, as deleting a client may orphan related car and intervention data unless cascade mechanisms are defined.
