using Lime.Protocol;
using Lime.Protocol.Serialization.Newtonsoft;
using Newtonsoft.Json;
using Server.Models;
using Server.Services.Interfaces;
using Take.Blip.Client;
using Take.Blip.Client.Activation;
using Take.Blip.Client.Web;

namespace Server.Services;

public class CommandService(EnvelopeSerializer serializer) : ICommandService
{
  private readonly Dictionary<string, ISender> _clientPool = [];

  public async Task<Command> SendAsync(ApplicationResponse application, Command command)
  {
    var client = GetSender(application.ShortName, application.AccessKey);

    var response = await client.ProcessCommandAsync(command, CancellationToken.None);

    return response;
  }

  public async Task<T?> SendAsync<T>(ApplicationResponse application, Command command) where T : class
  {
    var response = await SendAsync(application, command);

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
}
