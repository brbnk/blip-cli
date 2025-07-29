using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IBlipFunctionsHandler
{
    public Task<Response<object>> GetAsync(string identifier);
}