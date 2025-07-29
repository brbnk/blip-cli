using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IWorkingFlowHandler
{
    Task<Response<object>> GetAsync(string identifier);
}