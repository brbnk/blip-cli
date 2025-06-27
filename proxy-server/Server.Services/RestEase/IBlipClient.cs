using Lime.Protocol;
using RestEase;

namespace Server.Services.RestEase;

public interface IBlipClient
{
  [Header("Authorization")]
  public string Authorization { get; set; }


  [Post("/commands")]
  [Header("Content-Type", "application/json")]
  public Task<Command> SendCommandAsync([Body] Command command, CancellationToken cts);
}