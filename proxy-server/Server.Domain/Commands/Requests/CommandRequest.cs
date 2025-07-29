using Lime.Protocol;
using Newtonsoft.Json;

namespace Server.Domain.Commands.Requests;

public class CommandRequest(string identifier, Command? command)
{
    [JsonProperty("identifier")]
    public string Identifier { get; set; } = identifier;

    [JsonProperty("command")]
    public Command? Command { get; set; } = command;
}