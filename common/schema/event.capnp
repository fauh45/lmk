@0x8ed3139a54cca915;

interface EventInterface {
    struct Event {
        # Secret identifier shown on the website (e.g. rabbit-yellow-bandung)
        identifier @0 :Text;

        # milliseconds since 2024-01-17T15:00:00.000+00:00, because why should we go back in time?
        timestamp @1 :UInt64;

        struct Message {
            summary @0 :Text;
            body @1 :Text;
        }

        message @2 :Message;
    }

    trigger @0 (event: Event);
}