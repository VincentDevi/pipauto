# `Client` Table

The `client` table stores information about individuals who are customers of the car repair service. This includes identity, contact details, and metadata for indexing and search optimization. It is a key entity in the application, as clients are linked to their vehicles and interventions.


## Fields

| Field             | Type                | Description                                                                     |
| ----------------- | ------------------- | ------------------------------------------------------------------------------- |
| `first_name`      | `string`            | The first name of the client.                                                   |
| `last_name`       | `string`            | The last name of the client.                                                    |
| `full_name`       | `string` (computed) | Concatenation of `first_name` and `last_name`. Automatically updated via event. |
| `email`           | `option<string>`    | Optional email address. Defaults to `NONE`.                                     |
| `phone`           | `option<string>`    | Optional phone number. Defaults to `NONE`.                                      |
| `address`         | `object`            | An object containing full address information.                                  |
| `address.street`  | `string`            | Street name of the client’s address.                                            |
| `address.number`  | `string`            | Street number.                                                                  |
| `address.postal`  | `string`            | Postal code.                                                                    |
| `address.country` | `string`            | Country name.                                                                   |
| `created_at`      | `datetime`          | Timestamp when the record was created. Defaults to `time::now()`.               |
| `updated_at`      | `datetime`          | Timestamp when the record was last updated. Defaults to `time::now()`.          |

## Indexes

The following full-text search indexes are defined using a custom analyzer (client_analyzer) to improve lookup efficiency and relevance:

| Index Name          | Field        |
| ------------------- | ------------ |
| `client_email`      | `email`      |
| `client_phone`      | `phone`      |
| `client_first_name` | `first_name` |
| `client_last_name`  | `last_name`  |
| `client_full_name`  | `full_name`  |

All indexes use BM25 ranking and are optimized with caching strategies.


## Relationships

- **Owns Cars**: A client is linked to one or more records in the car table. Each car has a client field pointing back to its owner.
- **Indirect Interventions**: Through the cars they own, clients are also connected to intervention records. While interventions do not link directly to clients, they relate to cars, which are in turn owned by clients.

## Notes

- The client table uses full permissions for all fields, which means read/write access is unrestricted. This may be adjusted later for role-based access.
- The computed `full_name` ensures consistent display and efficient search.
-All timestamps are managed automatically using SurrealDB's `time::now()`.
