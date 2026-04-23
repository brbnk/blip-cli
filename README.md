# Introduction
Interact with blip flow locally. It's possible to create tests and mocks, visualize variables, trackings and scripts execution.

# Requirements
- bash
- pkg-config
- libssl-dev
- build-essential

# Setup
```bash
# Navigate to the blip-cli directory
$ cd blip-cli

# Make the setup script executable
$ chmod +x setup.sh

# Run the setup script to install dependencies and build the project
$ ./setup.sh

# Verify the installation
$ blip --version
```

# How to use
- Once installed, start de .NET proxy server
```bash
# Navigate to the proxy-server/Server.Api
$ cd proxy-server/Server.Api

# Start the project
$ dotnet run
```

- In another terminal, configure the BLIP_PORTAL_TOKEN environment variable
```bash
$ export BLIP_PORTAL_TOKEN=<token>
```

# Avaiable commands
```bash
$ blip --help

Usage: blip [COMMAND]

Commands:
  chat     simulate a chat
  mirror   mirror an application locally
  list     list local bots
  test     automate flow tests
  get      get data from blip
  analyze  scan flow
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## blip chat
- To simulate a chat locally, you need to download all bot resources from Blip, such as: 'working-flow', 'resources', 'blipfunctions', 'configurations', and 'global-actions'
- The _**blip mirror**_ command automates this process. For Routers, you can use:

```bash
$ blip mirror --tenant $TENANT --bot $BOT_ID --tier $CONTRACT_TIER --router
```

The command above will download the Router resources, iterate through all router child services, retrieve the required Blip resources, and store them in the _**~/.data/blip-cli**_ folder.

Possible values for $CONTRACT_TIER:
- Standard
- Business
- Enterprise
  
Now, it's possible to simulate a chat locally using:

```bash
$ blip chat --tenant $TENANT --bot $BOT_ID --router
```

### Demo:
**Blip Chat**

![blipchat_2040p](https://github.com/user-attachments/assets/5ceda8af-1b18-4d54-8da4-6296b9cc3272)

**Local Chat**

![chat_cli_2040p](https://github.com/user-attachments/assets/c72a2e61-e404-4483-943a-651b3a1566f5)

## blip mirror
- Download resources from Blip
```bash
$ blip mirror --help
mirror an application locally

Usage: blip mirror [OPTIONS] --tenant <TENANT> --bot <BOT>

Options:
      --tenant <TENANT>  contract
      --bot <BOT>        bot identifier
  -t, --tier <TIER>      tier contract
      --router           router applicatino
  -a, --all              mirror all
  -w, --working-flow     mirror only working flow
  -g, --global-actions   mirror only global actions
  -c, --configurations   mirror only config variables
  -b, --blip-functions   mirror only blip functions
  -r, --resources        mirror only resources
  -h, --help             Print help
```

## blip list
```bash
$ blip list

|- TenantExample
|--- botmain
|--- bottest
|--- botexceptions
```
