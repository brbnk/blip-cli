using System.Text;
using Lime.Protocol;
using Lime.Protocol.Serialization;
using Microsoft.AspNetCore.Mvc.Formatters;

namespace Server.Api.Formatters;

public class EnvelopeOutputFormatter : TextOutputFormatter
{
  private readonly IEnvelopeSerializer _envelopeSerializer;

  public EnvelopeOutputFormatter(IEnvelopeSerializer envelopeSerializer)
  {
    _envelopeSerializer = envelopeSerializer;
    SupportedMediaTypes.Add(Lime.Protocol.MediaType.ApplicationJson);
    SupportedEncodings.Add(Encoding.UTF8);
  }

  public override async Task WriteResponseBodyAsync(OutputFormatterWriteContext context, Encoding selectedEncoding)
  {
    var envelopeJson = _envelopeSerializer.Serialize(context.Object as Envelope);
    await context.HttpContext.Response.WriteAsync(envelopeJson);
  }

  protected override bool CanWriteType(Type? type)
  {
    return typeof(Envelope).IsAssignableFrom(type);
  }
}