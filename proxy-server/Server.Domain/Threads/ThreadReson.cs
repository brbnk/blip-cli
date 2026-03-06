using System.Text.Json.Serialization;

namespace Server.Domain.Threads;

public class ThreadReason
{
  [JsonPropertyName("code")]
  public int Code { get; set; }

  [JsonPropertyName("description")]
  public string? Description { get; set; } = string.Empty;
}