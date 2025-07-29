using Lime.Protocol;

namespace Server.Domain.Portal;

public interface IPortalCommandService
{
    void SetToken(string token);

    Task<Command> SendAsync(Command command);

    Task<T?> SendAsync<T>(Command command) where T : class;
}