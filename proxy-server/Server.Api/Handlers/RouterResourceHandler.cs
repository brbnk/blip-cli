using Lime.Protocol;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Commands.Responses;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Server.Domain.Shared.Constants;

namespace Server.Api.Handlers;

public sealed class RouterResourceHandler(ICommandService commandService) : IRouterResourceHandler
{
    private const string PATH = "resources";

    public async Task<Response<Dictionary<string, object?>>> GetAllAsync(string identifier)
    {
        var request = new CommandRequest(identifier, new() {
            Method = CommandMethod.Get,
            Uri = $"/{PATH}?$skip=0&$take=100",
            To = PostmasterConstants.POSTMASTER_MSGING
        });

        var resources = await commandService.SendAsync<CommandListResponse<string>>(request);
        if (resources is null || !resources.Items.Any())
        {
            return new()
            {
                Data = []
            };
        }

        // For each resource, get its value
        var tasks = resources.Items.Select(async resource =>
        {
            var value = await GetAsync(identifier, resource);
            return new KeyValuePair<string, object?>(resource, value.Data);
        });

        var results = await Task.WhenAll(tasks);

        return new()
        {
            Data = results.ToDictionary(k => k.Key, v => v.Value)
        };
    }

    public async Task<Response<object>> GetAsync(string identifier, string resource)
    {
        var request = new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = $"/{PATH}/{resource}",
            To = PostmasterConstants.POSTMASTER_MSGING
        });

        var response = await commandService.SendAsync<object>(request);

        return new()
        {
            Data = response
        };
    }
}