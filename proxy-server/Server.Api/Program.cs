using Lime.Protocol.Serialization;
using Lime.Protocol.Serialization.Newtonsoft;
using Microsoft.AspNetCore.Http.Json;
using Server.Api.Extensions;
using Server.Api.Formatters;
using Take.Blip.Client.Extensions;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
var documentResolver = new DocumentTypeResolver().WithBlipDocuments();

var serializer = new EnvelopeSerializer(documentResolver);
builder.Services.AddSingleton(serializer);
builder.Services.AddControllers(options =>
{
    options.InputFormatters.Insert(default, new EnvelopeInputFormatter(serializer));
    options.OutputFormatters.Insert(default, new EnvelopeOutputFormatter(serializer));
}).AddNewtonsoftJson();
builder.Services.AddServices();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection()
    .UseAuthentication()
    .UseRouting()
    .UseEndpoints(endpoints =>
    {
        endpoints.MapControllers();
    });

app.Run();
