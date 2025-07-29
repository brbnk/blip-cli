using Lime.Protocol;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Server.Domain.Shared.Constants;

namespace Server.Api.Handlers;

public sealed class ContextHandler(ICommandService commandService) : IContextHandler
{
    private const string PATH = "contexts";

    public async Task<Response<string>> GetAsync(string identifier, string contact, string context)
    {
        var request = new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = $"/{PATH}/{contact}/{context}",
            To = PostmasterConstants.POSTMASTER_BUILDER
        });

        var response = await commandService.SendAsync(request);

        return new()
        {
            Data = response?.Resource?.ToString()
        };
    }
}