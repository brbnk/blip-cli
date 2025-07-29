using System.Reflection.Emit;
using Lime.Protocol;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Server.Domain.Shared.Constants;

namespace Server.Api.Handlers;

public sealed class WorkingFlowHandler(ICommandService commandService) : IWorkingFlowHandler
{
    public async Task<Response<object>> GetAsync(string identifier)
    {
        var request = new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = "/buckets/blip_portal:builder_working_flow?$take=100",
            To = PostmasterConstants.POSTMASTER_MSGING
        });

        var response = await commandService.SendAsync(request);

        return new()
        {
            Data = new
            {
                Flow = response.Resource
            }
        };
    }
}
