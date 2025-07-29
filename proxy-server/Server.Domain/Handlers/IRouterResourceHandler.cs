using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IRouterResourceHandler
{
    Task<Response<Dictionary<string, object?>>> GetAllAsync(string identifier);

    Task<Response<object>> GetAsync(string identifier, string resource);
}