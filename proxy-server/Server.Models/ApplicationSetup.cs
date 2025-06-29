using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Server.Models;

public class BuilderConfiguration
{
  public string Application { get; set; } = string.Empty;

  public string Template { get; set; } = string.Empty;
}

public class BuilderSetup
{
  [JsonProperty("settings")]
  public BuilderSettings? Settings { get; set; }
}

public class BuilderSettings
{
  [JsonProperty("flow")]
  public FlowSettings? Flow { get; set; }
}

public class FlowSettings
{
  [JsonProperty("configuration")]
  public JToken? Configuration { get; set; }
}