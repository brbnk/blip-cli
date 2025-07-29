using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IContextHandler
{
    Task<Response<string>> GetAsync(string identifier, string contact, string context);
}