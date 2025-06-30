using Server.Models;
using Server.Services.Interfaces;

namespace Server.Services;

public class ResourceService(ICommandService commandService) : IResourceService
{
  public async Task<IDictionary<string, object?>> GetResources(ApplicationResponse application)
  {
    var resources = await commandService.SendAsync<CommandListResponse<string>>(
      application,
      CommandFactory.GetResourcesCommand());

    if (resources is null || !resources.Items.Any())
    {
      return new Dictionary<string, object?>();
    }

    var tasks = resources.Items.Select(async r =>
    {
      var t = await commandService.SendAsync<object>(application, CommandFactory.GetResourceCommand(r));
      return new KeyValuePair<string, object?>(r, t);
    });

    var results = await Task.WhenAll(tasks);

    return results.ToDictionary(k => k.Key, v => v.Value);
  }
}
