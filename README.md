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

The command above will download the Router resources, iterate through all router child services, retrieve the required Blip resources, and store them in the _**~/.blip-cli/data**_ folder.

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

- List all local bots retrieved by blip mirror command

```bash
$ blip list

|- TenantExample
|--- botmain
|--- bottest
|--- botexceptions
```

## blip test
- Create journey tests using mocks

```bash
$ blip test --help
automate flow tests

Usage: blip test <COMMAND>

Commands:
  create  create a template test file
  run     run tests
  help    Print this message or the help of the given subcommand(s)
```

- Create a template file with:
```bash
$ blip test create --tenant $TENANT --bot $BOT_ID
```
This command will create a file on '_**./blip-cli/data/$TENANT/$BOT_ID/__tests__**_' folder with the following structure

```json
{
  "description": "Test description",
  "inputs": [
    "olá",
    "sim"
  ],
  "mocks": {
    "myVar": "my value",
    "apiResponse": {
      "address": "Rua teste"
    }
  },
  "specs": {
    "ignoreCase": true
  },
  "asserts": [
    {
      "type": "variable",
      "variable": "exampleVar",
      "should": "BeEqual",
      "value": "value",
      "specs": null
    },
    {
      "type": "tracking",
      "category": "Category tracking examplo",
      "should": "BeEqual",
      "action": "Value tracking",
      "specs": null
    },
    {
      "type": "redirect",
      "service": "main",
      "should": "BeCalled",
      "withContextMessage": "redirect from child",
      "specs": null
    },
    {
      "type": "sendMessage",
      "message": "Olá, sou o bot Teste. Tudo bem com você?",
      "should": "BeEqual",
      "specs": null
    },
    {
      "type": "script",
      "outputVar": "exampleOutputScriptVar",
      "should": "BeEqual",
      "value": "en",
      "specs": null
    }
  ]
}
```

After, you can execute all tests with:

```bash
$ blip test run --tenant $TENANT --bot $BOT_ID
```

This is an example of output:

<img width="1157" height="899" alt="image" src="https://github.com/user-attachments/assets/9d51618b-5fe0-4038-8fc6-6e385d003a1a" />

## blip get
- Get data from blip

```bash
Usage: blip get <COMMAND>

Commands:
  key      bot auth key
  context  user context value
  thread   last user messages
  help     Print this message or the help of the given subcommand(s)
```

- Get the authorization key:
```bash
$ blip get key --bot $BOT_ID

#output
Key <auth_key>
```

- Get a context variable of a contact:
```bash
$ blip get context --bot <BOT_ID> --contact <CONTACT_IDENTITY> --variable <CONTEXT_VARIABLE>
```

- Get a contact conversaiont thread:
```bash
blip get thread --bot <BOT_ID> --contact <CONTACT_IDENTITY> | less
```

## blip analyze
- Count the number of actions like 'scripts', 'scriptv2', 'trackings', 'redirects', 'commands', 'processhttp' and states.
- Search across multiple flows

```bash
Usage: blip analyze [OPTIONS] --tenant <TENANT> --bot <BOT>

Options:
      --tenant <TENANT>  contract
      --bot <BOT>        bot identifier
  -r, --router
  -s, --scripts
  -f, --fetch
  -h, --http
  -c, --command
  -v, --variable
  -t, --tracking
  -r, --redirect
  -b, --blip-function
  -m, --merge-contacts
  -a, --agents
  -d, --desk
  -h, --help             Print help
```

Example of output without subcommands:

```bash
$ blip analyze --tenant $TENANT --bot $BOT_ID

#output
+----------------------------------------------------------+
|                          $BOT_ID                         |
+----------------------------------------------------------+
States = 24
ScriptV1 = 12
ScriptV2 = 40
ScriptV2_Http = 8
ProcessHttp = 2
ProcessCommand = 0
Trackings = 30
Variables = 40
Redirects = 2
BlipFunctions = 0
MergeContacts = 0
Agents = 0
Desk = 0
```

Example of output with the -v (--variable) subcommand. This will print all ocorrences of the searched variable.
Use '_--router_' to search on all Router services

```bash
$ blip analyze --tenant $TENANT --bot $BOT_ID -v --router

#output
+----------------------------------------------------------+
|                 <Router_child_identity>                  |
+----------------------------------------------------------+
States = 9
ScriptV1 = 2
ScriptV2 = 0
ScriptV2_Http = 0
ProcessHttp = 0
ProcessCommand = 0
Trackings = 6
Variables = 5
    State: MN.1.0.0 Menu Principal (9de26b3b-a562-4513-a05d-294036a91465) - set "isBackToStart" to true
    Variable: isBackToStart
    Value: true

    State: DV.1.0.0 Duvidas (0f21de5f-4d0b-490a-8cb9-e5ef16a0b715) - dv100
    Variable: dv100
    Value: {{input.content}}

    State: Exceções (fallback) - set "statePreviousId"
    Variable: statePreviousId
    Value: {{state.previous.id}}

    State: Início (onboarding) - reset "subfluxoRespostas"
    Variable: subfluxoRespostas
    Value: N/A

    State: salvar respostas (29438acb-1177-47c4-bf39-ce8e2229f720) - Definir variável
    Variable: subfluxoRespostas
    Value: true

Redirects = 0
BlipFunctions = 0
MergeContacts = 1
Agents = 0
Desk = 0
```
