using Lime.Protocol;

namespace Server.Models;

public static class CommandFactory
{
  public const string POSTMASTER_PORTAL = "postmaster@portal.blip.ai";
  public const string POSTMASTER_MSGING = "postmaster@msging.net";
  public const string POSTMASTER_CONFIGURATIONS = "postmaster@configurations.msging.net";
  public const string POSTMASTER_BUILDER = "postmaster@builder.msging.net";

  public static Command GetApplicationCommand(string identifier) => new()
  {
    Method = CommandMethod.Get,
    Uri = new LimeUri($"/applications/{identifier}@msging.net"),
    To = POSTMASTER_PORTAL
  };

  public static Command GetWorkingFlowCommand() => new()
  {
    Method = CommandMethod.Get,
    Uri = "/buckets/blip_portal:builder_working_flow?$take=100",
    To = POSTMASTER_MSGING
  };

  public static Command GetGlobalActionCommand() => new()
  {
    Method = CommandMethod.Get,
    Uri = "/buckets/blip_portal:builder_working_global_actions?$take=100",
    To = POSTMASTER_MSGING
  };

  public static Command GetBuilderConfigurationsCommand() => new()
  {
    Method = CommandMethod.Get,
    Uri = "/buckets/blip_portal:builder_working_configuration?$take=100",
    To = POSTMASTER_MSGING
  };

  public static Command GetResourcesCommand() => new()
  {
    Method = CommandMethod.Get,
    Uri = "/resources?$skip=0&$take=100",
    To = POSTMASTER_MSGING
  };

  public static Command GetResourceCommand(string key) => new()
  {
    Method = CommandMethod.Get,
    Uri = $"/resources/{key}",
    To = POSTMASTER_MSGING
  };

  public static Command GetBlipFunctions() => new()
  {
    Method = CommandMethod.Get,
    Uri = $"/functions",
    To = POSTMASTER_BUILDER
  };

  public static Command GetRouterChildren(string tier)
  {
    var uri = tier == "standard" ? "" : $"{tier}.";
    return new()
    {
      Method = CommandMethod.Get,
      Uri = $"lime://{uri}master.hosting@msging.net/configuration"
    };
  }

  public static Command GetContext(string contactIdentity, string context) => new()
  {
    Method = CommandMethod.Get,
    Uri = $"/contexts/{contactIdentity}/{context}",
    To = POSTMASTER_BUILDER
  };

  public static Command GetLastMessages(string contactIdentity) => new()
  {
    Method = CommandMethod.Get,
    Uri = $"/threads/{contactIdentity}?refreshExpiredMedia=true"
  };
}