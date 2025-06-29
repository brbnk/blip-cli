using Server.Models;

namespace Server.Services.Interfaces;

public interface IApplicationService
{
  Task<ApplicationResponse> GetAsync(string token, string identifier);
}