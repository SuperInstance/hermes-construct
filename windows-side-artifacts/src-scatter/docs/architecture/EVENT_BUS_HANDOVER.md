## Event Bus Handover (Part 1)

### Overview
The Event Bus is a central component designed to manage the flow of events within the system. It acts as a FIFO (First-In, First-Out) buffer with a fixed capacity.

### Core Components

#### `Event`
A simple structure representing a discrete piece of information.
- `id`: A unique identifier for the event.
- `payload`: The actual data being transmitted.

#### `EventBus`
The primary mechanism for ingesting and storing events.
- **Ingestion**: New events are added to the end of the queue.
- **Capacity Management**: The bus has a hard limit on the number of events it can hold.
- **Eviction Policy**: When a new event is pushed into a full bus, the oldest event (at the front of the queue) is automatically evicted to make room.

### Implementation Details
- The bus is implemented using `std::collections::VecDeque` for efficient $O(1)$ push and pop operations at both ends.
- Eviction follows a strict FIFO pattern.
