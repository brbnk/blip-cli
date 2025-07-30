using Lime.Protocol;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Server.Domain.Shared.Constants;

namespace Server.Api.Handlers;

public sealed class BuilderConfigurationsHandler(ICommandService commandService) : IBuilderConfigurationsHandler
{
    public async Task<Response<Dictionary<string, string>>> GetAsync(string identifier)
    {
        var request = new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = "/buckets/blip_portal:builder_working_configuration?$take=100",
            To = PostmasterConstants.POSTMASTER_MSGING
        });

        var configurations = await commandService.SendAsync<Dictionary<string, string>>(request);

        return new()
        {
            Data = configurations
        };
    }
}
