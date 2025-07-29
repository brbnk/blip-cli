using Microsoft.AspNetCore.Mvc;
using Server.Domain.Handlers;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/v1/application")]
public sealed class ApplicationController(
    IAdvancedSettingsHandler advancedSettingsHandler) : ControllerBase
{
    [HttpGet("key")]
    public async Task<IActionResult> GetAuthKey([FromQuery] string identifier)
    {
        return Ok(await advancedSettingsHandler.GetAuthKeyAsync(identifier));
    } 
}