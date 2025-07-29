using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.OpenApi.Models;
using Server.Domain.Commands;
using Server.Domain.Handlers;
using Server.Domain.Portal;
using Server.Domain.Shared.Constants;
using Server.Services;
using Server.Api.Handlers;

namespace Server.Api.Extensions;

public static class ServiceCollectionExtensions
{
    public static IServiceCollection AddServices(this IServiceCollection services)
    {
        services.AddSingleton<IPortalCommandService, PortalCommandService>();
        services.AddSingleton<ICommandService, BlipCommandService>();
        services.AddSingleton<IContextHandler, ContextHandler>();
        services.AddSingleton<IRouterResourceHandler, RouterResourceHandler>();
        services.AddSingleton<IWorkingFlowHandler, WorkingFlowHandler>();
        services.AddSingleton<IGlobalActionsHandler, GlobalActionsHandler>();
        services.AddSingleton<IBuilderConfigurationsHandler, BuilderConfigurationsHandler>();
        services.AddSingleton<IBlipFunctionsHandler, BlipFunctionsHandler>();
        services.AddSingleton<IAdvancedSettingsHandler, AdvancedSettingsHandler>();
        services.AddSingleton<IThreadHandler, ThreadHandler>();
        
        return services;
    }

    public static IServiceCollection AddSwaggerConfigurations(this IServiceCollection services)
    {
        services.AddSwaggerGen(c =>
        {
            c.SwaggerDoc("v1", new OpenApiInfo { Title = "Proxy Server", Version = "v1" });

            c.AddSecurityDefinition(JwtBearerDefaults.AuthenticationScheme,
                new OpenApiSecurityScheme
                  {
                      Name = SecurityConstants.ACCESS_TOKEN_HEADER,
                      Description = "Please enter a valid jwt token",
                      In = ParameterLocation.Header,
                      Type = SecuritySchemeType.ApiKey,
                      Scheme = JwtBearerDefaults.AuthenticationScheme
                  });
                  
            c.AddSecurityRequirement(new OpenApiSecurityRequirement
            {
                {
                    new OpenApiSecurityScheme
                    {
                        Reference = new OpenApiReference
                        {
                            Type = ReferenceType.SecurityScheme,
                            Id = JwtBearerDefaults.AuthenticationScheme
                        }
                    },
                    []
                }
            });
        });

        services.AddSwaggerGenNewtonsoftSupport();

        return services;
    }
}