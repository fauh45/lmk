@0x8ed3139a54cca915;

struct Event {
    # Secret identifier shown on the website (e.g. rabbit-yellow-bandung)
    identifier @0 :Text;
    # Secret code accompanying the identifier, used to make sure user doesn't brute force through identifier
    secretCode @1 :UInt16;

    # milliseconds since 2024-01-17T15:00:00.000+00:00, because why should we go back in time?
    timestamp @2 :UInt64;

    struct Message {
        summary @0 :Text;
        body @1 :Text;
    }
}