namespace Server.Models;

public class CommandListResponse<T>
{
  public int Total { get; set; }
  
  public string ItemType { get; set; } = string.Empty;
  
  public IEnumerable<T> Items { get; set; } = Enumerable.Empty<T>();
}