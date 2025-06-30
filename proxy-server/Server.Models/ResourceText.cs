using System.Runtime.Serialization;
using Lime.Protocol;

namespace Server.Models;

[DataContract]
public class ResourceText : Document
{
  public const string MIME_TYPE = "text/plain";

  public static readonly MediaType MediaType = MediaType.Parse(MIME_TYPE);

  public ResourceText() : base(MediaType)
  {
  }

  [DataMember(Name = "value")]
  public string Value { get; set; } = string.Empty;

  [DataMember(Name = "text")]
  public string Text { get; set; } = string.Empty;
}