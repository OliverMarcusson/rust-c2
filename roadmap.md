## **C2 Framework Roadmap (TCP Foundation, Iterative Build-up)**

### **Phase 0: Prep & Planning**
- Research existing open-source (Red Team) C2s (e.g., Sliver, Mythic, Covenant) for context and inspiration.
- Create a new Rust multi-binary workspace with shared types crate (`common`).
- Set minimal, testable milestones per phase.

---

### **Phase 1: Minimal Viable C2 Core**
**Goal:** Prove end-to-end connectivity and task roundtrip.

#### **A. Shared: Define Protocol & Messages**
   - Create a `proto` crate with shared Rust enums/structs for agent/server messages.
     - E.g., `Message::Hello`, `Message::Task(TaskType)`, `Message::Result(Data)`.

#### **B. Server**
   - Listens on TCP port for incoming agent connections (`tokio`, `async_std`, or `std::net` for simple version).
   - Accepts one agent at first.
   - Prints: Connection events, incoming messages.

#### **C. Agent**
   - Simple binary that connects to server TCP socket, sends a `Hello` message, waits for a `Task`, executes a stub, sends fake result back.

#### **D. Operator Client**
   - Separate binary OR server handles commands from stdin for now.
   - Send basic task (e.g., `TaskType::Shell("whoami")`) to connected agent.
   - Print result as received.

#### **E. Success Criteria**
   - Server->Agent: “whoami” (simulated task)
   - Agent->Server: “root\n” (simulated result)
   - Manual operator tasking via CLI/in-code trigger.

---

### **Phase 2: Multi-Agent, Session Tracking, Simple CLI**
**Goal:** Add multi-agent support, basic operator experience.

#### **Features**
- Server: Accepts multiple agents, assigns each a session/conn ID.
- Server: Tracks online agents, allows targeting for tasking.
- Server: Basic CLI (type `list`, `task <sess> <cmd>`) using `clap`, `reedline` or just stdin loop.
- Agent: Handles reconnects/export new connection if dropped.

---

### **Phase 3: Bi-Directional Task Queue, Real Command Execution**
**Goal:** Real tasking and process execution.

#### **Features**
- Messages: Agent regularly polls server for pending tasks (“beacon”), accepts response.
- Operator: Tasks are queued per agent, dispatched on check-in.
- Agent: Actually spawns subprocess for `Shell(cmd)` task (`std::process::Command`), collects stdout/stderr.
- Security: Basic errors handled so agent doesn’t crash on bad commands.

---

### **Phase 4: Message Serialization, Refined Protocol**
**Goal:** Robustness, versioning.

#### **Features**
- Use binary serialization (`bincode`, `serde_cbor`) or JSON (`serde_json`) instead of raw strings.
- Define message framing: length-prefixed messages over TCP to separate packets.
- Start a protocol version mechanism in messages, so future changes are possible.

---

### **Phase 5: Encrypted Transport & Basic Authentication**
**Goal:** Increase operational security.

#### **Features**
- Add TLS to comms (self-signed for now, `rustls` or `native-tls`).
- Agent authenticates to server with shared secret or token in Hello message.
- Operator authentication (basic password/TOTP) at CLI.

---

### **Phase 6: Error Handling, Logging, Persistence**
**Goal:** Production-safe.

#### **Features**
- Robust error handling (timeouts, errors, reconnections).
- Structured logging (`tracing`, `log`).
- Persist agent session state, task history (MVP: to disk as JSON, next: DB).

---

### **Phase 7: Modular Tasking & Extensibility**
**Goal:** Easy addition of new capabilities.

#### **Features**
- Implement plugin/task trait system (so new task types can be added easily).
- Upgrade agent/server to handle new, arbitrary task payloads.
- Start integrating common ops modules: file upload/download, screenshot, etc.

---

### **Phase 8+: Advanced Features & Transport Options**
**Goal:** Approaching production C2 capability.

#### **Features**
- Multiple transports: Add HTTP(S)/DNS/Covert Channels alongside TCP.
- Operator Web UI (optional): Web server integration.
- Evasion OpSec features: Traffic randomization/jitter, staging, in-memory execution, etc.
- Lateral movement, credential theft modules, persistent agent implant features.

---

## **Example Milestone Summary Table**

| Phase | Key Deliverables                        | Rust Crate Hints                 | Unit Test/Success Check          |
|-------|-----------------------------------------|----------------------------------|----------------------------------|
| 1     | TCP server/agent, roundtrip task/result | `tokio`, `serde`, `bincode`      | Server prints agent results      |
| 2     | Multi-agent sessions, server CLI        | `clap`, `reedline`               | Operator can task any agent      |
| 3     | Real command exec, polling beacon       | `std::process`, error handling   | Output from real command visible |
| 4     | Protocol serialization, framing         | `serde_json`, `bincode`          | Structured messages over TCP     |
| 5     | TLS transport, authentication           | `rustls`, `ring`, `jsonwebtoken` | Only agents with right token on  |
| 6     | Logs, error handling, persistence       | `tracing`, `serde`, `sled`       | Survives restart, logs actions   |
| 7     | Modular tasking                        | traits, dynamic dispatch         | Easily add/enable new tasks      |

---

