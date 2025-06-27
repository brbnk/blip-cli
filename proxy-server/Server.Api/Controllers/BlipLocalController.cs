using Lime.Protocol;
using Microsoft.AspNetCore.Mvc;
using Server.Models;
using Server.Services.Interfaces;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class BlipLocalController(IPortalService portalService, ICommandService commandService) : ControllerBase
{
  [HttpGet("flow")]
  public async Task<IActionResult> GetWorkingFlowAsync([FromHeader] string token, [FromHeader] string identifier)
  {
    var response = await portalService.SendAsync<ApplicationCommandResponse>(token, new Command
    {
      Method = CommandMethod.Get,
      Uri = new LimeUri($"/applications/{identifier}@msging.net"),
      To = "postmaster@portal.blip.ai"
    });

    if (response is null)
    {
      return BadRequest($"Não foi possível buscar as informações do idenfier {identifier}");
    }

    var flow = await commandService.SendAsync(response.ShortName, response.AccessKey, new Command
    {
      Method = CommandMethod.Get,
      Uri = "/buckets/blip_portal:builder_working_flow?$take=100",
      To = "postmaster@msging.net"
    });

    var globaActions = await commandService.SendAsync(response.ShortName, response.AccessKey, new Command
    {
      Method = CommandMethod.Get,
      Uri = "/buckets/blip_portal:builder_working_global_actions?$take=100",
      To = "postmaster@msging.net"
    });

    return Ok(new
    {
      Flow = flow.Resource,
      GlobalActions = globaActions.Resource
    });
  }
}