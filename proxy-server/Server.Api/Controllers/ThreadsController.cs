using Microsoft.AspNetCore.Mvc;
using Server.Domain.Handlers;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/v1/threads")]
public sealed class ThreadsController(IThreadHandler handler) : ControllerBase
{
    [HttpGet]
    public async Task<IActionResult> GetThreadAsync([FromHeader] string contact, [FromQuery] string identifier)
    {
        return Ok(await handler.GetAsync(identifier, contact));
    }
}