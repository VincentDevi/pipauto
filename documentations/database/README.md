# SurrealDB Database Documentation

## Overview

This document provides a comprehensive analysis of the SurrealDB database designed for automotive service management. The database manages clients, their vehicles, maintenance interventions, and ownership history. This system appears to be designed for an automotive service center or garage that tracks customer vehicles and their service history.

## Database Architecture

### Schema Configuration
- **Database Type**: SurrealDB with SCHEMAFULL tables
- **Permissions**: Currently set to NONE for all tables
- **Analyzer**: Custom `client_analyzer` with TOKENIZERS CLASS,CAMEL and FILTERS NGRAM(1,4),LOWERCASE for search functionality

## Tables Detailed Analysis

#### 1. Client Table

- **Purpose**: Manages individuals associated with vehicle ownership.
- **Fields**:
  - `first_name`, `last_name`, `full_name`: Identification.
  - `address`: Structured as a nested object with street, number, postal, and country details.
  - `email`, `phone`: Optional fields for contact.
  - `created_at`, `updated_at`: Timestamps documenting record changes.
- **Relationships**:
  - Links with the Car table through `client_id`, enabling multiple car records per client.
  - Indirectly tied to interventions via car ownership.

#### 2. Car Table

- **Purpose**: Details cars and their characteristics.
- **Fields**:
  - `brand`, `model`, `fuel`, `year`: Car attributes.
  - `cc`: Engine capacity.
  - `oil_type`, `oil_quantity`: Attributes pivotal for maintenance.
  - `client_id`: Connects the car to its owner in the Client table.
- **Relationships**:
  - Car-client associations via `client_id`, supporting ownership tracking.
  - Associations with Intervention table signify service history.

#### 3. Intervention Table

- **Purpose**: Chronicles maintenance or repairs of vehicles.
- **Fields**:
  - `car_id`: Connects to the Car table, specifying interventions per vehicle.
  - `client`: Relates back to the client linked with the car.
  - `intervention_date`: Date when service actions occurred.
  - `intervention_type`: Either a `Maintenance` object or a `Repair` label.
  - `mileage`, `price`: Details on vehicle condition and service costs.
- **Relationships**:
  - Tied to cars, indicating all performed interventions and cumulative service history.
  - Influences potential vehicle resale strategies and service schedules.

#### 4. Car Owner History Table

- **Purpose**: Envisioned to log ownership chronology.
- **Status**: Currently unused with no data, suggesting underdevelopment or incomplete integration.

### Relationship Implications

- **Client-Car Tie**: Enhances ownership tracking, beneficial for client relationship management and downstream services.
- **Car-Intervention Connection**: Provides a detailed historical service account, critical for maintenance and operational analytics.
- **Fuel Diversity**: Different fuel types necessitate diverse service and parts needs, affecting operational logistics and inventory.
- **Historical Trends**: Engine sizes and car models by year can influence strategic decisions on inventory and marketing.

This documentation aims to provide intricate insights into the database's structure and relationships, essential for maintaining a robust system aligned with business strategies.
