using System.Text;
using Lime.Protocol;
using Lime.Protocol.Serialization;
using Microsoft.AspNetCore.Mvc.Formatters;

namespace Server.Api.Formatters;

public class EnvelopeInputFormatter : TextInputFormatter
{
  private readonly IEnvelopeSerializer _envelopeSerializer;

  public EnvelopeInputFormatter(IEnvelopeSerializer envelopeSerializer)
  {
    _envelopeSerializer = envelopeSerializer;
    SupportedMediaTypes.Add(Lime.Protocol.MediaType.ApplicationJson);
    SupportedEncodings.Add(Encoding.UTF8);
  }
  
  public override async Task<InputFormatterResult> ReadRequestBodyAsync(InputFormatterContext context, Encoding encoding)
  {
    using var reader = new StreamReader(context.HttpContext.Request.Body, encoding);
    var body = await reader.ReadToEndAsync();
    var envelope = _envelopeSerializer.Deserialize(body);
    return InputFormatterResult.Success(envelope);
  }

  protected override bool CanReadType(Type type)
  {
    return typeof(Envelope).IsAssignableFrom(type);
  }
}