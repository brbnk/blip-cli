using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Server.Models;

public class ApplicationCommandResponse
{
  public string Application { get; set; } = string.Empty;

  public string Template { get; set; } = string.Empty;
}

public class ApplicationSetup
{
  [JsonProperty("settings")]
  public ApplicationSettings? Settings { get; set; }
}

public class ApplicationSettings
{
  [JsonProperty("flow")]
  public FlowSettings? Flow { get; set; }
}

public class FlowSettings
{
  [JsonProperty("configuration")]
  public JToken? Configuration { get; set; }
}