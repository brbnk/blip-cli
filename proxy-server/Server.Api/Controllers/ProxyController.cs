using Microsoft.AspNetCore.Mvc;
using Server.Models;
using Server.Services.Interfaces;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class ProxyController(IPortalService portalService, ICommandService commandService) : ControllerBase
{
  [HttpGet("data")]
  public async Task<IActionResult> GetWorkingFlowAsync([FromHeader] string token, [FromHeader] string identifier)
  {
    var application = await portalService.SendAsync<ApplicationResponse>(
      token,
      CommandFactory.GetApplicationCommand(identifier));

    if (application is null)
    {
      return BadRequest($"Não foi possível buscar as informações do identifier {identifier}");
    }

    var flow = await commandService.SendAsync(
      application,
      CommandFactory.GetWorkingFlowCommand());

    var globaActions = await commandService.SendAsync(
      application,
      CommandFactory.GetGlobalActionCommand());

    return Ok(new
    {
      Flow = flow.Resource,
      GlobalActions = globaActions.Resource
    });
  }
}