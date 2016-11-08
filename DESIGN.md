The overall design of the RustBucket bot will be a mixture of factories
and MVC + Command Pattern. 

# Model
The individual commands for the bot as well as the Messages
That may be sent from/to the Telegram Bot API will make up
the models of the Bot. Models will be built using factories,
and configuration to handle such will be done through a
configuration factory with configuration modules injected as
dependencies.

##Bot Commands
A Bot Command should be invoked as /command args, internally
commands will be grouped logically by struct/module, so we need to
have the Controller searching by going through a list of list of
commands, and invoking the execute function with the parameters
passed to it. 

A Bot Command should also have priveleges associated with the struct
in order to prevent unauthorized users from invoking administrative
commands.

A Bot Command should implement a trait BotCommand or something similar
that will act as a common entry point, this trait must have the following
functions at a minimum:

    * getCommands() -> List<String>
    * execute(command : String, arguments : String, user : UserInfo)

UserInfo should contain available information about the user (name and such)
as well as permission information, and the origin of the user so that we can
discriminate whether a command should be run based upon whether it comes from
telegram, the local host, or other sources. In the configuration this should
probably be handled as a list of allowed hosts for a module.

Ideally we should be able to extend the bot capabilities of the bot
just by writing a bot command and configuration module and injecting
the configuration module into the configurator.

# View
The view is the http/https socket which will run in it's own thread
and will both feed messages to the controller, and receive messages 
from the controller. There are two views, one to connect to telegram
another to connect to the localhost web configuration frontend. It
would be a good idea to review the concepts of a bot and create an
internal representation of a command flow, and then translate to/from
that to the Telegram Bot API to allow for future extensibility. This
will also allow us to create an integration server to test the commands
of the bot as opposed to having to play games with Telegram.

# Controller
The controller will recieve messages from the view and then use
those messages to select a command to run.

The controller should support a security infrastructure to recognize
users that message it and grant tiered priveleged access as follows:

Priveleges:

    1. Localhost
    2. Bot owners
    3. Group Owners / Admins of the current group it's in
    4. Normal Users

This may be expanded later to support mods separate from group owners
and voiced users separate from normal users (in matching with the IRC
schema) as this may be eventually transformed into a generic bot framework
that supports arbitrary services.

# Applications
## The Bot
The bot is the actual core of the application and will be run on
a hosted server.

## The Web Config Frontend
There shall be a web configuration frontend accessible on a local
port on the machine that is running RustBucket, settings will operate
temporarily (until the bot is restarted) unless saved, and will have
the capability to restore to a sane default configuration. 
