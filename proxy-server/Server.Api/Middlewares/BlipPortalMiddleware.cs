using Server.Domain.Portal;
using Server.Domain.Shared.Constants;

namespace Server.Api.Middlewares;

public class BlipPortalMiddleware(RequestDelegate next)
{
    public async Task InvokeAsync(HttpContext context)
    {
        if (context.Request.Headers.TryGetValue(SecurityConstants.ACCESS_TOKEN_HEADER, out var token))
        {
            var service = context.RequestServices.GetRequiredService<IPortalCommandService>();
            service.SetToken(token!);
        }
        await next(context);
    }
}