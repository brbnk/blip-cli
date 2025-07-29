using Newtonsoft.Json;

namespace Server.Domain.BlipFunctions;

public class BlipFunction
{
  [JsonProperty("functionId")]
  public string FunctionId { get; set; } = string.Empty;

  [JsonProperty("functionName")]
  public string FunctionName { get; set; } = string.Empty;

  [JsonProperty("functionDescription")]
  public string FunctionDescription { get; set; } = string.Empty;

  [JsonProperty("functionContent")]
  public string FunctionContent { get; set; } = string.Empty;

  [JsonProperty("tentantId")]
  public string TentantId { get; set; } = string.Empty;

  [JsonProperty("userIdentity")]
  public string UserIdentity { get; set; } = string.Empty;
}