using System.Text.Json.Serialization;
using Newtonsoft.Json.Linq;

namespace Server.Domain.Threads;

public class Thread
{
  [JsonPropertyName("id")]
  public string? Id { get; set; } = string.Empty;

  [JsonPropertyName("direction")]
  public string? Direction { get; set; } = string.Empty;

  [JsonPropertyName("type")]
  public string? Type { get; set; } = string.Empty;

  [JsonPropertyName("content")]
  public JToken? Content { get; set; }

  [JsonPropertyName("date")]
  public string? Date { get; set; } = string.Empty;

  [JsonPropertyName("status")]
  public string? Status { get; set; } = string.Empty;

  [JsonPropertyName("metadata")]
  public IDictionary<string, string>? Metadata { get; set; }
}