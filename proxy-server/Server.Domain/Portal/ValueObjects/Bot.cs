using System.Text;
using Newtonsoft.Json;

namespace Server.Domain.Portal.ValueObjects;

public class Bot
{
  [JsonProperty("shortName")]
  public string ShortName { get; set; } = string.Empty;

  [JsonProperty("name")]
  public string? Name { get; set; }

  [JsonProperty("description")]
  public string? Description { get; set; }

  [JsonProperty("accessKey")]
  public string AccessKey { get; set; } = string.Empty;

  [JsonProperty("template")]
  public string? Template { get; set; }

  [JsonProperty("created")]
  public string? Crated { get; set; }

  [JsonProperty("updated")]
  public string? Updated { get; set; }

  [JsonProperty("hasPermission")]
  public bool? HasPermission { get; set; }

  [JsonProperty("emailOwner")]
  public string? EmailOwner { get; set; }

   public string GetAuthorizationKey()
  {
    if (string.IsNullOrWhiteSpace(ShortName) || string.IsNullOrWhiteSpace(AccessKey))
    {
      return string.Empty;
    }

    var decoded = Encoding.UTF8.GetString(Convert.FromBase64String(AccessKey));
    var authorization = $"{ShortName}:{decoded}";

    return $"Key {Convert.ToBase64String(Encoding.UTF8.GetBytes(authorization))}";
  }
}