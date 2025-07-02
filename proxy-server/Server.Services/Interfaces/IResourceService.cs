using Server.Models;

namespace Server.Services.Interfaces;

public interface IResourceService
{
  Task<IDictionary<string, object?>> GetResources(ApplicationResponse application);
}