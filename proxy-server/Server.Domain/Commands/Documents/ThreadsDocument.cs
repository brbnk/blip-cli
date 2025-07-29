using System.Runtime.Serialization;
using Lime.Protocol;
using Thread = Server.Domain.Threads.Thread;

namespace Server.Domain.Commands.Documents;

[DataContract]
public class ThreadsDocument : Document
{
  public const string MIME_TYPE = "application/vnd.lime.collection+json";

  public static readonly MediaType MediaType = MediaType.Parse(MIME_TYPE);

  public ThreadsDocument() : base(MediaType)
  {
  }

    
  [DataMember(Name = "total")]
  public int Total { get; set; }

  [DataMember(Name = "itemType")]
  public string ItemType { get; set; } = string.Empty;

  [DataMember(Name = "items")]
  public IEnumerable<Thread> Items { get; set; } = [];
}