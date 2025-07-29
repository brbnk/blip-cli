using Newtonsoft.Json;

namespace Server.Domain.Application;

public class RouterSettings
{
  [JsonProperty("settings")]
  public RouterChildren? Settings { get; set; }
}