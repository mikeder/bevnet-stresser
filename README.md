# bevnet-stresser
A stress test for various Bevy networking libraries. 

This repo will attempt to implement the most basic server authoritative examples from each repo tested and run through
 some common game networking scenarios.

1. Replicating components/entities from server to client.
   1. Spawn and replicate an increasing number of entities from the server to the client.
2. Server authoritative movement of spawn entities.
   1. Send position data for each visible entity to all clients.
   2. (Stretch) Implement "interest management" or "spatial indexing" to send only *relevant* entites to each client.
3. Custom message passing, client to server and server to client.
   1. Unidirectional and Bidirectional message passing.
   2. Quick/small messages vs. large messages.
   3. (Stretch) Implement different serialization methods (bincode vs. bitcode)

During each of these scenarios, performance of the client and server will be measured:

1. FPS
2. Frame time
3. Server network throughput
4. Client(s) network throughput
5. (Stretch) Rollback Rate


## Libraries to be tested

- [ ] [bevy_replicon](https://github.com/projectharmonia/bevy_replicon)
- [ ] [lightyear](https://github.com/cBournhonesque/lightyear)
- [ ] [renet](https://github.com/lucaspoffo/renet)

