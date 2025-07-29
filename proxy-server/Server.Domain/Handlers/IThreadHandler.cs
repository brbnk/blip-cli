using Server.Domain.Shared;
using Thread = Server.Domain.Threads.Thread;

namespace Server.Domain.Handlers;

public interface IThreadHandler
{
    Task<Response<IEnumerable<Thread>>> GetAsync(string identifier, string contact);
}