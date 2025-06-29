using Server.Models;
using Server.Services.Interfaces;

namespace Server.Services;

public class ApplicationService(IPortalService portalService) : IApplicationService
{
  private readonly static Dictionary<string, ApplicationResponse> _cache = [];

  public async Task<ApplicationResponse> GetAsync(string token, string identifier)
  {
    if (_cache.TryGetValue(identifier, out var application))
    {
      return application;
    }

    var response = await portalService.SendAsync<ApplicationResponse>(
      token,
      CommandFactory.GetApplicationCommand(identifier));

    ArgumentNullException.ThrowIfNull(response, "");

    _cache.Add(identifier, response);

    return response;
  }
}
