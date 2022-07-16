# Contacts App WIP
A simple app that stores contacts.

## Tech
### Implemented
- Yew: front-end component based framework.  Like React
- Rocket: back-end web framework, much slower than other frameworks (like actix), but much easier to use and understand.
- Diesel: Sql ORM, maps Sql commands to functions.  Adds a lot of useful functionality.

### Unimplemeted
- Tauri: creates a desktop application from a webpage.

## TODO
1. Add update and delete features
2. Add nullables, easy just add options to struct
3. Style
4. implement Tauri framework
5. implement favorites
6. implement active, instead of deleting from db, just set contact as inactive
7. Auto fetch contacts when page first loads
