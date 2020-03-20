# uno-rs
UnoD project by Shankar, Mario and Alex.

### Game Rules
Fully implemented with testing in master branch

### XMPP functionality
Couldn't get Prosody to work as server, so we've implemented the XMPP server functionality under the *xmpp branch*.

Since it was implemented using the code for running the game locally, there's a lot of functionality that wasn't finished as it would require a complete change in design. You'd need to have 4 copies of game state that work independently (as this is what you'd have over a server). The main functionality is there and the core functions have been tested (those functions not needing a TcpStream).

Stay safe and thank you for a great quarter!