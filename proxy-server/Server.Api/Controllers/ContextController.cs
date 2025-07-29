using Microsoft.AspNetCore.Mvc;
using Server.Domain.Handlers;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/v1/context")]
public class ContextController(IContextHandler handler) : ControllerBase
{
    [HttpGet]
    public async Task<IActionResult> GetContextAsync(
        [FromHeader] string contact,
        [FromQuery] string identifier,
        [FromQuery] string context)
    {
        return Ok(await handler.GetAsync(identifier, contact, context));
    }
}