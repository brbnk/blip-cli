using Microsoft.AspNetCore.Mvc;
using Server.Domain.Enums;
using Server.Domain.Handlers;

namespace Server.Api.Controllers;

[ApiController]
[Route("api/v1/mirror")]
public sealed class MirrorController(
    IWorkingFlowHandler workingFlow,
    IGlobalActionsHandler globalActions,
    IBuilderConfigurationsHandler builderConfigs,
    IBlipFunctionsHandler blipFunctions,
    IAdvancedSettingsHandler advancedSettingsHandler) : ControllerBase
{
    [HttpGet("working-flow")]
    public async Task<IActionResult> GetWorkingFlowAsync([FromQuery] string identifier)
    {
        return Ok(await workingFlow.GetAsync(identifier));
    }

    [HttpGet("global-actions")]
    public async Task<IActionResult> GetGlobalActionsAsync([FromQuery] string identifier)
    {
        return Ok(await globalActions.GetAsync(identifier));
    }

    [HttpGet("builder-configs")]
    public async Task<IActionResult> GetBuilderConfigurationsAsync([FromQuery] string identifier)
    {
        return Ok(await builderConfigs.GetAsync(identifier));
    }

    [HttpGet("blip-functions")]
    public async Task<IActionResult> GetBlipFunctionsAsync([FromQuery] string identifier)
    {
        return Ok(await blipFunctions.GetAsync(identifier));
    }

    [HttpGet("router-children")]
    public async Task<IActionResult> GetRouterChildrenAsync([FromQuery] string identifier, [FromQuery] ContractTier tier)
    {
        return Ok(await advancedSettingsHandler.GetRouterChildrenAsync(identifier, tier));
    }
}