using Newtonsoft.Json;

namespace Server.Models;

public sealed class CommandResource<T> where T : class
{
  [JsonProperty("resource")]
  public T? Resource { get; set; }
}