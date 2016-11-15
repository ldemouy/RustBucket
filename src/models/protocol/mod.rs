mod protocols {
    enum Command {
        SendMessage { recepient: String, message: String },
        SendCommand { command: String },
        Join { channel: String },
        Leave { channel: String },
        KeepAlive,
        OpenConnection,
        CloseConnection,
    }
    enum PermissionLevel {
        Any = 0,
        User = 1,
        VoiceUser = 2,
        Moderator = 3,
        Admin = 4,
        BotOwner = 5,
        Localhost = 6,
    }
    enum Protocol {
        Telegram,
        IRC,
        Any,
    }
    struct UserData {
        name: String,
        permission_level: PermissionLevel,
        channel: String, // may be empty
        origin: String,
        protocol: Protocol,
    }
}
