using Lime.Protocol.Serialization;
using Lime.Protocol.Serialization.Newtonsoft;
using Newtonsoft.Json.Converters;
using Server.Api.Extensions;
using Server.Api.Formatters;
using Server.Api.Handlers.Exceptions;
using Server.Api.Middlewares;
using Server.Domain.Commands.Documents;
using Take.Blip.Client.Extensions;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
var documentResolver = new DocumentTypeResolver().WithBlipDocuments();
documentResolver.RegisterDocument(typeof(ResourceCollectionDocument));
documentResolver.RegisterDocument(typeof(BuilderFunctionsDocument));

var serializer = new EnvelopeSerializer(documentResolver);
builder.Services.AddSingleton(serializer);

builder.Services.AddControllers(options =>
{
    options.InputFormatters.Insert(default, new EnvelopeInputFormatter(serializer));
    options.OutputFormatters.Insert(default, new EnvelopeOutputFormatter(serializer));
})
.AddNewtonsoftJson(options =>
{
    options.SerializerSettings.Converters.Add(new StringEnumConverter());
});

builder.Services.AddServices();
builder.Services.AddExceptionHandler<GlobalExceptionHandler>();
builder.Services.AddProblemDetails();
builder.Services.AddSwaggerConfigurations();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseExceptionHandler();
app.UseMiddleware<BlipPortalMiddleware>();

app.UseHttpsRedirection()
    .UseAuthentication()
    .UseRouting()
    .UseEndpoints(endpoints =>
    {
        endpoints.MapControllers();
    });

app.Run();
