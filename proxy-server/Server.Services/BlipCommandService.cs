using Lime.Protocol;
using Lime.Protocol.Serialization.Newtonsoft;
using Newtonsoft.Json;
using Server.Domain.Commands;
using Server.Domain.Commands.Requests;
using Server.Domain.Commands.Responses;
using Server.Domain.Portal;
using Server.Domain.Portal.ValueObjects;
using Server.Domain.Shared.Constants;
using Take.Blip.Client;
using Take.Blip.Client.Activation;
using Take.Blip.Client.Web;

namespace Server.Services;

public sealed class BlipCommandService(
    EnvelopeSerializer serializer,
    IPortalCommandService portalService) : ICommandService
{
    private readonly Dictionary<string, ISender> _clientPool = [];
    private readonly static Dictionary<string, Bot> _applications = [];

    public async Task<Command> SendAsync(CommandRequest request)
    {
        var application = await GetApplicationAsync(request.Identifier);
        var sender = GetSender(application.ShortName, application.AccessKey);
        var response = await sender.ProcessCommandAsync(request.Command, CancellationToken.None);
        return response;
    }

    public async Task<T?> SendAsync<T>(CommandRequest request) where T : class
    {
        var response = await SendAsync(request);

        if (response is null)
        {
            return null;
        }

        if (response.Type == MediaType.TextPlain)
        {
            var text = response?.Resource.ToString();
            return text as T;
        }

        var json = JsonConvert.SerializeObject(response);
        var resource = JsonConvert.DeserializeObject<CommandResource<T>>(json);

        return resource?.Resource;
    }

    public async Task<string> GetAuthKey(string identifier)
    {
        var application = await GetApplicationAsync(identifier);
        return application.GetAuthorizationKey();
    }

    #region Private methods

    private ISender GetSender(string identifier, string accessKey)
    {
        var id = identifier.Trim();

        if (_clientPool.TryGetValue(id, out var sender))
        {
            return sender;
        }

        var application = new Application
        {
            Identifier = identifier,
            AccessKey = accessKey,
            HostName = "msging.net",
            Port = 443,
            Scheme = Uri.UriSchemeHttps
        };

        var envelopeBuffer = new EnvelopeBuffer();
        var webTransportFactory = new WebTransportFactory(envelopeBuffer, serializer, application);
        var client = new BlipClientBuilder(webTransportFactory)
          .UsingAccessKey(application.Identifier, application.AccessKey)
          .UsingScheme(application.Scheme)
          .UsingHostName(application.HostName)
          .Build();

        _clientPool.Add(id, client);

        return client;
    }

    private async Task<Bot> GetApplicationAsync(string identifier)
    {
        if (_applications.TryGetValue(identifier, out var application))
        {
            return application;
        }

        var bot = await portalService.SendAsync<Bot>(new()
        {
            Method = CommandMethod.Get,
            Uri = new LimeUri($"/applications/{identifier}@msging.net"),
            To = PostmasterConstants.POSTMASTER_PORTAL
        });

        ArgumentNullException.ThrowIfNull(bot, "");

        _applications.Add(identifier, bot);

        return bot;
    }

    #endregion
}
