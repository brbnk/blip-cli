using Newtonsoft.Json;

namespace Server.Domain.Application;

public class RouterChild
{
  [JsonProperty("identity")]
  public string Identity { get; set; } = string.Empty;

  [JsonProperty("isDefault")]
  public bool IsDefault { get; set; }

  [JsonProperty("longName")]
  public string JsonPropertyLongName { get; set; } = string.Empty;

  [JsonProperty("service")]
  public string Service { get; set; } = string.Empty;

  [JsonProperty("shortName")]
  public string ShortName { get; set; } = string.Empty;
}