using Newtonsoft.Json;

namespace Server.Domain.Application;

public class RouterChildren
{
  [JsonProperty("children")]
  public IEnumerable<RouterChild> Children { get; set; } = [];
}