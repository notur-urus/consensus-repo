//How It Works//

Each vote has a timestamp.
Older votes are weighted less (decay).
A result is chosen if any option passes the threshold.

//Usage//

cargo run -- --votes A,B,A,A --decay exp

//Modules//

decay.rs: Vote weakening logic.
threshold.rs: Changing threshold rules.
vote.rs: Time window and vote structure.
consensus.rs: Main consensus logic.

//Unit Tests//

Consensus result when majority is strong enough.
No result if threshold isn’t met.
Decay works: newer votes outweigh older ones.
Vote count tracking.
Voting window closes after timeout.

