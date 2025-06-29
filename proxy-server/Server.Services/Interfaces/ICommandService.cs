using Lime.Protocol;
using Server.Models;

namespace Server.Services.Interfaces;

public interface ICommandService
{
  Task<Command> SendAsync(ApplicationResponse application, Command command);

  Task<T?> SendAsync<T>(ApplicationResponse application, Command command) where T : class;
}