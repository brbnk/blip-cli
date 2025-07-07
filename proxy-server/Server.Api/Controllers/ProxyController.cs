using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json;
using Server.Models;
using Server.Services.Interfaces;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class ProxyController(
  IApplicationService applicationService,
  ICommandService commandService,
  IResourceService resourceService) : ControllerBase
{
  [HttpGet("working-flow")]
  public async Task<IActionResult> GetWorkingFlowAsync([FromHeader] string token, [FromQuery] string identifier)
  {
    var application = await applicationService.GetAsync(token, identifier);
    var flow = await commandService.SendAsync(application, CommandFactory.GetWorkingFlowCommand());

    return Ok(new
    {
      Application = new
      {
        Flow = flow.Resource
      }
    });
  }

  [HttpGet("global-actions")]
  public async Task<IActionResult> GetGlobalActionsAsync([FromHeader] string token, [FromQuery] string identifier)
  {
    var application = await applicationService.GetAsync(token, identifier);
    var globalActions = await commandService.SendAsync(application, CommandFactory.GetGlobalActionCommand());

    return Ok(new
    {
      GlobalActions = globalActions.Resource
    });
  }

  [HttpGet("configs")]
  public async Task<IActionResult> GetBuilderConfigurationsAsync([FromHeader] string token, [FromQuery] string identifier)
  {
    var application = await applicationService.GetAsync(token, identifier);
    var configurations = await commandService.SendAsync<IDictionary<string, string>>(application, CommandFactory.GetBuilderConfigurationsCommand());

    return Ok(new
    {
      Configurations = configurations
    });
  }

  [HttpGet("resources")]
  public async Task<IActionResult> GetResourcesAsync([FromHeader] string token, [FromQuery] string identifier)
  {
    var application = await applicationService.GetAsync(token, identifier);
    var resources = await resourceService.GetResources(application);

    return Ok(new
    {
      Resources = resources
    });
  }

  [HttpGet("blip-functions")]
  public async Task<IActionResult> GetBlipFunctionAsync([FromHeader] string token, [FromQuery] string identifier)
  {
    var application = await applicationService.GetAsync(token, identifier);

    var response = await commandService.SendAsync<CommandListResponse<BuilderFunction>>(
      application,
      CommandFactory.GetBlipFunctions());

    return Ok(new
    {
      Tenant = response?.Items.FirstOrDefault()?.TentantId,
      Functions = response?.Items.Select(i =>
      {
        return new
        {
          Id = i.FunctionId,
          Name = i.FunctionName,
          Content = i.FunctionContent,
        };
      })
    });
  }

  [HttpGet("router")]
  public async Task<IActionResult> GetRouterAsync([FromHeader] string token, [FromQuery] string routerId, [FromQuery] string tier)
  {
    var application = await applicationService.GetAsync(token, routerId);

    var response = await commandService.SendAsync<BuilderConfiguration>(
      application,
      CommandFactory.GetRouterChildren(tier));

    if (response?.Application is null)
    {
      throw new ArgumentException("application is null");
    }

    var children = JsonConvert.DeserializeObject<RouterSetup>(response.Application);

    return Ok(children?.Settings?.Children ?? []);
  }
}