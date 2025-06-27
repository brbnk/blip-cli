using Lime.Protocol;

namespace Server.Services.Interfaces;

public interface IPortalService
{
  Task<Command> SendAsync(string token, Command command);

  Task<T?> SendAsync<T>(string token, Command command) where T : class;
}