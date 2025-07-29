namespace Server.Domain.Commands.Responses;

public sealed class CommandResource<T> where T : class
{
  public T? Resource { get; set; }
}