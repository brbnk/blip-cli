using Lime.Protocol;
using Newtonsoft.Json;
using Server.Domain.Application;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Enums;
using Server.Domain.Handlers;
using Server.Domain.Shared;

namespace Server.Api.Handlers;

public sealed class AdvancedSettingsHandler(ICommandService commandService) : IAdvancedSettingsHandler
{
    public async Task<Response<string>> GetAuthKeyAsync(string identifier)
    {
        return new()
        {
            Data = await commandService.GetAuthKey(identifier)
        };
    }

    public async Task<Response<IEnumerable<RouterChild>>> GetRouterChildrenAsync(string identifier, ContractTier tier)
    {
        var uri = tier switch
        {
            ContractTier.Standard => "master.hosting",
            ContractTier.Business => "business.master.hosting",
            ContractTier.Enterprise => "enterprise.master.hosting",
            _ => "master.hosting"
        };

        var response = await commandService.SendAsync<AdvancedSettings>(new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = $"lime://{uri}@msging.net/configuration"
        }));

        if (response?.Application is null)
        {
            return new()
            {
                Success = false,
                Data = [],
                Message = $"Advanced settings not found for {identifier}"
            };
        }

        var children = JsonConvert.DeserializeObject<RouterSettings>(response.Application);

        return new()
        {
            Data = children?.Settings?.Children ?? []
        };
    }
}
