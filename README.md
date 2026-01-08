# Rust TCP Chat Server Implementation

Your task is to implement a TCP chat server using Rust and `tokio`. The server should handle multiple concurrent clients and support specific commands for interaction.

## Requirements

1.  **Server Setup**:
    -   The server must listen on `127.0.0.1:8080`.
    -   It should accept multiple client connections concurrently.

2.  **Commands**:
    The server must parse and handle the following commands from clients:

    -   `/nickname <name>`:
        -   Updates the client's nickname.
        -   **Validation**: The nickname must not contain spaces.
        -   **Action**: Notify all connected clients that the user has entered the chat (e.g., `<name> entered the chat`).
        -   **Response**: Send a welcome message to the client (e.g., `Hello, <name>`).

    -   `/broadcast <message>`:
        -   Sends a message to all connected clients.
        -   **Format**: The message should be prefixed with the sender's nickname (e.g., `<nickname>: <message>`).

    -   `/exit [message]`:
        -   Disconnects the client.
        -   **Optional Argument**: A parting message.
        -   **Action**: Notify all connected clients that the user has left.
            -   If a message is provided: `<nickname> exited the chat with <message>`
            -   If no message is provided: `<nickname> exited the chat`

3.  **Error Handling**:
    -   Handle invalid commands or missing parameters gracefully (e.g., print errors to stderr or send feedback to the client, though the reference implementation mainly logged errors).
    -   Ensure the server doesn't crash on client disconnects or malformed input.

## Technical Details

-   Use `tokio::net::TcpListener` for the server.
-   Use `tokio::sync::broadcast` to handle message broadcasting to all clients.
-   Use `Arc<Mutex<...>>` where necessary for shared state (though `broadcast` might alleviate some need for this regarding message passing).
-   Clients should be handled in their own asynchronous tasks (`tokio::spawn`).

## Good Luck!
