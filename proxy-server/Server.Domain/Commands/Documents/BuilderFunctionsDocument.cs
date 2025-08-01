using System.Runtime.Serialization;
using Lime.Protocol;
using Server.Domain.BlipFunctions;

namespace Server.Domain.Commands.Documents;

[DataContract]
public class BuilderFunctionsDocument : Document
{
  public const string MIME_TYPE = "application/vnd.iris.builder.function+json";

  public static readonly MediaType MediaType = MediaType.Parse(MIME_TYPE);

  public BuilderFunctionsDocument() : base(MediaType)
  {
  }

  
  [DataMember(Name = "total")]
  public int Total { get; set; }

  [DataMember(Name = "itemType")]
  public string ItemType { get; set; } = string.Empty;

  [DataMember(Name = "items")]
  public IEnumerable<BlipFunction> Items { get; set; } = [];
}