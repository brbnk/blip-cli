using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IBuilderConfigurationsHandler
{
    public Task<Response<Dictionary<string, string>>> GetAsync(string identifier);
}