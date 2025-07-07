using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Server.Models;

public class BuilderConfiguration
{
  public string Application { get; set; } = string.Empty;

  public string Template { get; set; } = string.Empty;
}

public class RouterSetup
{
  [JsonProperty("settings")]
  public RouterChildren? Settings { get; set; }
}

public class RouterChildren
{
  [JsonProperty("children")]
  public IEnumerable<RouterChild> Children { get; set; } = [];
}

public class RouterChild
{
  [JsonProperty("identity")]
  public string Identity { get; set; } = string.Empty;

  [JsonProperty("isDefault")]
  public bool IsDefault { get; set; }

  [JsonProperty("longName")]
  public string LongName { get; set; } = string.Empty;

  [JsonProperty("service")]
  public string Service { get; set; } = string.Empty;

  [JsonProperty("shortName")]
  public string ShortName { get; set; } = string.Empty;
}