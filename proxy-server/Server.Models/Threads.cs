using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Server.Models;

public class Threads
{
  [JsonProperty("id")]
  public string? Id { get; set; } = string.Empty;

  [JsonProperty("direction")]
  public string? Direction { get; set; } = string.Empty;

  [JsonProperty("type")]
  public string? Type { get; set; } = string.Empty;

  [JsonProperty("content")]
  public JToken? Content { get; set; }

  [JsonProperty("date")]
  public string? Date { get; set; } = string.Empty;

  [JsonProperty("status")]
  public string? Status { get; set; } = string.Empty;

  [JsonProperty("metadata")]
  public IDictionary<string, string>? Metadata { get; set; }
}