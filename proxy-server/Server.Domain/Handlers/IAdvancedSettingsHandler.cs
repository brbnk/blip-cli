using Server.Domain.Application;
using Server.Domain.Enums;
using Server.Domain.Shared;

namespace Server.Domain.Handlers;

public interface IAdvancedSettingsHandler
{
    public Task<Response<IEnumerable<RouterChild>>> GetRouterChildrenAsync(string identifier, ContractTier tier);

    public Task<Response<string>> GetAuthKeyAsync(string identifier);
}