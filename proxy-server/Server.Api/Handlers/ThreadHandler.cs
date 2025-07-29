using Lime.Protocol;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Commands.Responses;
using Server.Domain.Handlers;
using Server.Domain.Shared;
using Thread = Server.Domain.Threads.Thread;

namespace Server.Api.Handlers;

public sealed class ThreadHandler(ICommandService commandService) : IThreadHandler
{
    public async Task<Response<IEnumerable<Thread>>> GetAsync(string identifier, string contact)
    {
        var request = new CommandRequest(identifier, new()
        {
            Method = CommandMethod.Get,
            Uri = $"/threads/{contact}?refreshExpiredMedia=true"
        });

        var response = await commandService.SendAsync<CommandListResponse<Thread>>(request);

        return new()
        {
            Data = response?.Items ?? []
        };
    }
}
