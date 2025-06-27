using Newtonsoft.Json;

namespace Server.Models;

public class User(string email, string fullName)
{
  [JsonProperty("email")]
  public string Email { get; set; } = email ?? string.Empty;

  [JsonProperty("fullName")]
  public string FullName { get; set; } = fullName ?? string.Empty;

  [JsonProperty("identity")]
  public string Identity { get; set; } = string.Empty;
}