use std::io::{self, BufRead, BufReader, Write};

#[cfg(windows)]
pub const IPC_SOCKET_NAME: &str = "StickyNote_IPC";

#[cfg(not(windows))]
pub const IPC_SOCKET_NAME: &str = "/tmp/StickyNote_IPC.sock";

/// IPC commands exchanged between secondary instances / Settings and the primary instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcCommand {
    /// Open, unhide, and focus a note file by path.
    OpenFile(String),
    /// Spawn a new cascaded sticky note.
    NewNote,
    /// Reload application settings from `config.json`.
    ReloadConfig,
}

impl IpcCommand {
    /// Format command for transmission over the local IPC channel.
    pub fn to_wire(&self) -> String {
        match self {
            Self::OpenFile(path) => format!("OPEN_FILE {path}\n"),
            Self::NewNote => "NEW_NOTE\n".to_string(),
            Self::ReloadConfig => "RELOAD_CONFIG\n".to_string(),
        }
    }

    /// Parse a wire command string received from IPC.
    pub fn parse(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if trimmed == "NEW_NOTE" {
            Some(Self::NewNote)
        } else if trimmed == "RELOAD_CONFIG" {
            Some(Self::ReloadConfig)
        } else {
            trimmed
                .strip_prefix("OPEN_FILE ")
                .map(|path| Self::OpenFile(path.trim().to_string()))
        }
    }
}

/// Returns directory-scoped IPC socket name to prevent cross-directory instance hijacking.
pub fn get_ipc_socket_name() -> String {
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_string_lossy().to_string()))
        .unwrap_or_else(|| "default".to_string());
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&base, &mut hasher);
    let hash = std::hash::Hasher::finish(&hasher);
    format!("StickyNote_IPC_{:016x}", hash)
}

/// Send a command to the primary running instance.
/// Returns Ok(true) if connected and delivered, or Ok(false) if no instance is running.
pub fn send_ipc_command(cmd: &IpcCommand) -> io::Result<bool> {
    use interprocess::local_socket::prelude::*;
    use interprocess::local_socket::{GenericNamespaced, Stream, ToNsName};

    let sock_name = get_ipc_socket_name();
    let name = match sock_name.as_str().to_ns_name::<GenericNamespaced>() {
        Ok(n) => n,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
    };

    let mut stream = match Stream::connect(name) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound || e.kind() == io::ErrorKind::ConnectionRefused => {
            return Ok(false);
        }
        Err(e) => return Err(e),
    };

    let wire = cmd.to_wire();
    stream.write_all(wire.as_bytes())?;
    stream.flush()?;
    Ok(true)
}

/// Listener for incoming IPC connections on the primary instance.
pub struct IpcServer {
    listener: interprocess::local_socket::Listener,
}

impl IpcServer {
    /// Bind the IPC server endpoint. Returns Ok(None) if another instance is already running.
    pub fn bind() -> io::Result<Option<Self>> {
        use interprocess::local_socket::{GenericNamespaced, ListenerOptions, ToNsName};

        let sock_name = get_ipc_socket_name();
        let name = match sock_name.as_str().to_ns_name::<GenericNamespaced>() {
            Ok(n) => n,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
        };

        match ListenerOptions::new().name(name).create_sync() {
            Ok(listener) => Ok(Some(Self { listener })),
            Err(e) if e.kind() == io::ErrorKind::AddrInUse => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Accept incoming connection and read all commands from it.
    pub fn accept_commands<F>(&self, mut handler: F) -> io::Result<()>
    where
        F: FnMut(IpcCommand),
    {
        use interprocess::local_socket::prelude::*;

        for conn in self.listener.incoming().flatten() {
            let mut reader = BufReader::new(conn);
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line) {
                if n == 0 {
                    break;
                }
                if let Some(cmd) = IpcCommand::parse(&line) {
                    handler(cmd);
                }
                line.clear();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_command_wire_and_parse() {
        let open_cmd = IpcCommand::OpenFile("C:\\Notes\\Note 1.json".to_string());
        assert_eq!(open_cmd.to_wire(), "OPEN_FILE C:\\Notes\\Note 1.json\n");
        assert_eq!(
            IpcCommand::parse("OPEN_FILE C:\\Notes\\Note 1.json\n"),
            Some(open_cmd)
        );

        let new_cmd = IpcCommand::NewNote;
        assert_eq!(new_cmd.to_wire(), "NEW_NOTE\n");
        assert_eq!(IpcCommand::parse("NEW_NOTE"), Some(new_cmd));

        let reload_cmd = IpcCommand::ReloadConfig;
        assert_eq!(reload_cmd.to_wire(), "RELOAD_CONFIG\n");
        assert_eq!(IpcCommand::parse("RELOAD_CONFIG\r\n"), Some(reload_cmd));

        assert_eq!(IpcCommand::parse("UNKNOWN_CMD"), None);
    }
}
