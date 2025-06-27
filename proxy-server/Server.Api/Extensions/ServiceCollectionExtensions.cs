using Lime.Protocol.Serialization;
using Lime.Protocol.Serialization.Newtonsoft;
using Server.Api.Formatters;
using Server.Services;
using Server.Services.Interfaces;
using Take.Blip.Client.Extensions;

namespace Server.Api.Extensions;

public static class ServiceCollectionExtensions
{
  public static IServiceCollection AddServices(this IServiceCollection services)
  {
    services.AddSingleton<IPortalService, PortalService>();
    services.AddSingleton<ICommandService, CommandService>();

    return services;
  }
}