using Microsoft.AspNetCore.Mvc;
using Server.Domain.Handlers;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/v1/resource")]
public sealed class ResourceController(IRouterResourceHandler handler) : ControllerBase
{
    [HttpGet]
    public async Task<IActionResult> GetAsync([FromQuery] string identifier, [FromQuery] string resource)
    {
        return Ok(await handler.GetAsync(identifier, resource));
    }

    [HttpGet("all")]
    public async Task<IActionResult> GetResourcesAsync([FromQuery] string identifier)
    {
        return Ok(await handler.GetAllAsync(identifier));
    }
}