using System.Runtime.Serialization;
using Lime.Protocol;

namespace Server.Models;

[DataContract]
public class ResourceCollectionDocument : Document
{
  public const string MIME_TYPE = "application/vnd.lime.collection+json";

  public static readonly MediaType MediaType = MediaType.Parse(MIME_TYPE);

  public ResourceCollectionDocument() : base(MediaType)
  {
  }

  [DataMember(Name = "total")]
  public int Total { get; set; }

  [DataMember(Name = "itemType")]
  public string ItemType { get; set; } = string.Empty;

  [DataMember(Name = "items")]
  public IEnumerable<string> Items { get; set; } = [];
}