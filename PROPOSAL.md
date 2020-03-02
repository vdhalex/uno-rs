# Uno.rs

### The Product
As a team, we plan to build a CLI implementation of the card game Uno. It will be an online game for 2-4 players. The deck will be shuffled and the cards distributed among the players who will each play one at time; until there's a winner.

### Must haves
- A 2 player online game, with scalability up to 4. 
- A shuffling mechanism.
- Some form of encryption to make sure players can't cheat and look at the deck. 
- Timeout features in case a player disconnects or is AFK for too long which will either boot them from the game and add their cards to the deck and shuffle, or proclaim the last player victor.

### Reaches
- Multifactor encryption to ensure maximum amount of security.
- Authentication for players.
- Database which will keep track of player's past games (coupled with the auth).

### Difficulties
- Finding a good way to encrypt this non trivially but in a way that we can implement (at our level of understanding).
- A good, fair and as random as possible shuffle implementation.
- Working with possible microservices might be trickier than we think.