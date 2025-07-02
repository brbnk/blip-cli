using Server.Services;
using Server.Services.Interfaces;

namespace Server.Api.Extensions;

public static class ServiceCollectionExtensions
{
  public static IServiceCollection AddServices(this IServiceCollection services)
  {
    services.AddSingleton<IPortalService, PortalService>();
    services.AddSingleton<ICommandService, CommandService>();
    services.AddSingleton<IApplicationService, ApplicationService>();
    services.AddSingleton<IResourceService, ResourceService>();

    return services;
  }
}