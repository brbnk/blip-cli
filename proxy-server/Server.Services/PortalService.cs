using Lime.Protocol;
using Lime.Protocol.Listeners;
using Lime.Protocol.Serialization.Newtonsoft;
using Lime.Protocol.Util;
using Newtonsoft.Json;
using Server.Models;
using Server.Services.Extensions;
using Server.Services.Interfaces;
using Take.Blip.Client;

namespace Server.Services;

public sealed class PortalService(EnvelopeSerializer envelopeSerializer) : IPortalService
{
  private const string DEFAULT_ISSUER = "account.blip.ai";
  private IBlipClient? _client;

  public async Task<Command> SendAsync(string token, Command command)
  {
    var client = await GetClientAsync(token);

    var response = await client.ProcessCommandAsync(command, CancellationToken.None);

    return response;
  }

  public async Task<T?> SendAsync<T>(string token, Command command) where T : class
  {
    var response = await SendAsync(token, command);

    if (response is null)
    {
      return null;
    }

    var json = JsonConvert.SerializeObject(response);
    var resource = JsonConvert.DeserializeObject<CommandResource<T>>(json);

    return resource?.Resource;
  }

  private async Task<IBlipClient> GetClientAsync(string token)
  {
    if (_client is not null)
    {
      return _client;
    }

    var user = token.GetUser();

    _client = new BlipClientBuilder(new TcpTransportFactory(envelopeSerializer))
      .UsingHostName("tcp.blip.ai")
      .UsingDomain("blip.ai")
      .UsingInstance(user.Email)
      .UsingEncryption(SessionEncryption.TLS)
      .UsingExternal(user.Email.Replace("@", "%40"), token, DEFAULT_ISSUER)
      .WithPresenceStatus(Lime.Messaging.Resources.PresenceStatus.Available)
      .Build();

    var channelListenner = new ChannelListener(
      (c, mts) => TaskUtil.TrueCompletedTask,
      (n, ctx) => TaskUtil.TrueCompletedTask,
      (c, cts) => TaskUtil.TrueCompletedTask);

    await _client.StartAsync(channelListenner, CancellationToken.None);

    return _client;
  }
}
