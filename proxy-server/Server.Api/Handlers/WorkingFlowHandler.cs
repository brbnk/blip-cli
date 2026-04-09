using System.Text.RegularExpressions;
using Lime.Protocol;
using Newtonsoft.Json;
using Server.Domain.Application;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Server.Domain.Shared.Constants;

namespace Server.Api.Handlers;

public sealed partial class WorkingFlowHandler(ICommandService commandService) : IWorkingFlowHandler
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

    public async Task<Response<object>> GetSubflowAsync(string flowId)
    {
        var flow = await GetAsync(flowId);

        var workingFlow = JsonConvert.SerializeObject(flow.Data);

        var matches = ShortNameOfSubflowRegex().Matches(workingFlow ?? string.Empty);

        var subflowIds = matches
            .Cast<Match>()
            .Select(m => m.Groups[1].Value)
            .ToList();

        var subflows = new Dictionary<string, string>();

        foreach (var subflowId in subflowIds)
        {
            var request = new CommandRequest(flowId, new()
            {
                Method = CommandMethod.Get,
                Uri = $"/subflows/{subflowId}/configurations/edited-flow",
                To = PostmasterConstants.POSTMASTER_BUILDER
            });

            var response = await commandService.SendAsync(request);

            var subflow = JsonConvert.DeserializeObject<WorkingSubflow>(JsonConvert.SerializeObject(response.Resource));

            subflows.Add(subflowId, JsonConvert.SerializeObject(new { 
                flow = JsonConvert.DeserializeObject<object>(subflow.Document) 
            }));
        }


        return new()
        {
            Data = subflows
        };
    }

    [GeneratedRegex(@"""shortNameOfSubflow""\s*:\s*""([^""]+)""")]
    private static partial Regex ShortNameOfSubflowRegex();
}
