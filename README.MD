# Contacts App WIP
- A simple app that stores contacts.

- I started by making an API that makes CRUD operations on a database that stores all of the contacts.
- Next I made a website that calls simple get/post requests to display and manipulate the database.

## Tech
### Implemented
- Yew: front-end component based framework.  Like React
- Rocket: back-end web framework, much slower than other frameworks (like actix), but much easier to use and understand.
- Diesel: Sql ORM, maps Sql commands to functions.  Adds a lot of useful functionality.
- Tailwind: CSS framework.

### Unimplemeted
- Tauri: creates a desktop application from a webpage.

## TODO
- fix email and phone number edit
- clean up emails and phone numbers
- Add image support
- combine search bars into one
- auto refresh
- implement Tauri frameworks
- Auto fetch contacts when page first loads
