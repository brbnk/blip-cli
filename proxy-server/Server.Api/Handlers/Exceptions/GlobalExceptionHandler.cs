using System.Net;
using Microsoft.AspNetCore.Diagnostics;
using Server.Domain.Shared;

namespace Server.Api.Handlers.Exceptions;

public class GlobalExceptionHandler : IExceptionHandler
{
    public async ValueTask<bool> TryHandleAsync(HttpContext httpContext, Exception exception, CancellationToken cancellationToken)
    {
        var response = new Response()
        {
            Success = false,
            Message = exception?.InnerException?.Message ?? "Something went wrong",
            ErrorMessage = exception?.Message,
        };

        httpContext.Response.StatusCode = (int)HttpStatusCode.InternalServerError;

        await httpContext.Response.WriteAsJsonAsync(response, cancellationToken);

        return true;
    }
}
