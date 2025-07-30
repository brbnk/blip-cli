namespace Server.Domain.Shared;

public class Response
{
    public bool Success { get; set; } = true;

    public string? Message { get; set; }

    public string? ErrorMessage { get; set; }
}

public class Response<T> : Response where T : class
{
    public T? Data { get; set; }
} 