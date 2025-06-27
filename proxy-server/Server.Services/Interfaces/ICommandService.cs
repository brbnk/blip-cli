using Lime.Protocol;

namespace Server.Services.Interfaces;

public interface ICommandService
{
  Task<Command> SendAsync(string identifier, string accessKey, Command command);

  Task<T?> SendAsync<T>(string identifier, string accessKey, Command command) where T : class;
}