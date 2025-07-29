using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IGlobalActionsHandler
{
    public Task<Response<object>> GetAsync(string identifier);
}