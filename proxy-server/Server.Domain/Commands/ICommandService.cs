using Lime.Protocol;
using Server.Domain.Commands.Requests;

namespace Server.Domain.Commands;

public interface ICommandService
{
    Task<string> GetAuthKey(string identifier);

    Task<Command> SendAsync(CommandRequest request);

    Task<T?> SendAsync<T>(CommandRequest request) where T : class;
}