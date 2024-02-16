# Surplus Distribution System

## Overview
This Rust program implements a simple application to allow vendors to dispose of their surplus food effectively to those in need. It also provides functionality to search for vendors, manage excess food, and add new excess items.

## Components

### Data Structures
- **Vendor**: Represents a cafe, hotel, or restaurant record containing fields such as ID, name, location, phone, and optional update timestamp.
- **Excess**: Represents all excess produce used for adding or updating excess items.
- **Error**: An enum representing various error types that can occur during operations, including not found, validation error, and insertion failure.

### Storage
- **STORAGE**: A thread-local stable B-tree map storing vendor and excess food records keyed by their unique IDs.
- **MEMORY_MANAGER**: Manages memory for the application.
- **ID_COUNTER**: Generates unique identifiers for vendor and excess food records.

### Functions
- **get_vendor_from_id**: Retrieves a vendor record by its ID.
- **get_excess_from_id**: Retrieves an excess food record by its ID.
- **add_vendor**: Adds a new vendor record to the system.
- **add_excess**: Adds a new excess food record to the system.
- **update_vendor**: Updates an existing vendor record.
- **delete_vendor**: Deletes a vendor record from the system.
- **delete_excess**: Deletes an excess food record from the system.

### Helper Functions
- **do_insert**: Inserts a vendor record into the storage.
- **do_insert_excess**: Inserts an excess food record into the storage.
- **_get_vendor**: Retrieves a vendor record by its ID.
- **_get_excess**: Retrieves an excess food record by its ID.

## Candid Interface
The program exports a Candid interface for interaction with the Internet Computer.

## Usage
1. Add a vendor record using `add_vendor`.
2. Add an excess food record using `add_excess`.
3. Retrieve a vendor record by its ID using `get_vendor_from_id`.
4. Retrieve an excess food record by its ID using `get_excess_from_id`.
5. Update an existing vendor record using `update_vendor`.
6. Delete a vendor record using `delete_vendor`.
7. Delete an excess food record using `delete_excess`.

## Error Handling
The program handles various error scenarios such as not found, validation errors, and insertion failures, providing informative error messages.

## Running the Project Locally

To test the project locally, you can follow these steps:

1. Start the replica running in the background:
```bash
$ dfx start --background
