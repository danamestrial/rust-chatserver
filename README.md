# Rust-chatserver
ICCS 311 Project <br/>
## Project name
Circle OpenChat  — Code name: <b>Don’t think, just do </b>

## Running the project
### Backend
Requirements to run backend: `Docker`, `postgresql` & `libqp-dev` for Diesel Cli <br/>
#### Database
Once inside the backend folder do (in the order given): <br/>
`./make-db.sh {password}` script to create a Docker instance <br/>
#### Diesel
`cargo install diesel_cli --no-default-features --features postgres` to install Diesel Cli <br/>
`diesel setup` <br/>
`diesel migration run` <br/>
Then `cargo run` to start the backend

### Frontend
Once inside the frontend folder do (in the order given): <br/>
`yarn install` <br/>
then `yarn serve` to start the frontend
  
## Project Plan
To build a chat web server that has features like, ‘ Line OpenchatTM ‘,  but with other features like grouping, tagging, ability to have different roles and more. <br/>
  
## Project features/Scope
* Starting the framework of the chat room
* Designing/Creating *beautiful* GUIs for it
* Setting the role for each user
* Tagging people by calling their name
* Ability to write messages, (and other people in the room sees in)
* Ability to ‘login’ as an instructor/TA
* Inviting people to a group via code and/or link
* Ability to connect and access a room and remove people
* Persistent messages in a room (storing all the messages in a db and the ability to parse everything back when the room is restarted)

## Prototype
Our team created a interactive prototype of our chat server project on Figma:
https://www.figma.com/proto/l9W7tZHApIOcqplj6tWWtP/Circle-Chat-Server?node-id=19%3A10&scaling=contain&page-id=0%3A1&starting-point-node-id=19%3A10
