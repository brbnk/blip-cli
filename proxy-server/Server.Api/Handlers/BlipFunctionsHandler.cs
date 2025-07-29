using Lime.Protocol;
using Server.Domain.BlipFunctions;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Commands.Responses;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Server.Domain.Shared.Constants;

namespace Server.Api.Handlers;

public sealed class BlipFunctionsHandler(ICommandService commandService) : IBlipFunctionsHandler
{
    public async Task<Response<object>> GetAsync(string identifier)
    {
        var request = new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = $"/functions",
            To = PostmasterConstants.POSTMASTER_BUILDER
        });
        
        var response = await commandService.SendAsync<CommandListResponse<BlipFunction>>(request);

        return new()
        {
            Data = new
            {
                Tenant = response?.Items.FirstOrDefault()?.TentantId,
                Functions = response?.Items.Select(i =>
                {
                    return new
                    {
                        Id = i.FunctionId,
                        Name = i.FunctionName,
                        Content = i.FunctionContent,
                    };
                })
            }
        };
}
}
