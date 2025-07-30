using Lime.Protocol;
using Lime.Protocol.Listeners;
using Lime.Protocol.Serialization.Newtonsoft;
using Lime.Protocol.Util;
using Newtonsoft.Json;
using Server.Domain.Commands.Responses;
using Server.Domain.Portal;
using Server.Services.Extensions;
using Take.Blip.Client;

namespace Server.Services;

public sealed class PortalCommandService(EnvelopeSerializer serializer) : IPortalCommandService
{
    private const string DEFAULT_ISSUER = "account.blip.ai";
    private IBlipClient? _client;
    private string _token = string.Empty;

    public async Task<Command> SendAsync(Command command)
    {
        var client = await GetClientAsync(_token);
        return await client.ProcessCommandAsync(command, CancellationToken.None);
    }

    public async Task<T?> SendAsync<T>(Command command) where T : class
    {
        var response = await SendAsync(command);

        if (response is null)
        {
            return null;
        }

        var json = JsonConvert.SerializeObject(response);
        var resource = JsonConvert.DeserializeObject<CommandResource<T>>(json);

        return resource?.Resource;
    }

    public void SetToken(string token)
    {
        _token = token;
    }

    #region Private methods

    private async Task<IBlipClient> GetClientAsync(string token)
    {
        if (_client is not null)
        {
            return _client;
        }

        var user = token.GetUser();

        _client = new BlipClientBuilder(new TcpTransportFactory(serializer))
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

    #endregion
}
