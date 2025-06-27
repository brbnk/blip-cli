using System.IdentityModel.Tokens.Jwt;
using Server.Models;

namespace Server.Services.Extensions;

public static class StringExtensions
{
  public static User GetUser(this string token)
  {
    var handler = new JwtSecurityTokenHandler();

    var jwtToken = handler.ReadJwtToken(token);

    var email = jwtToken
      .Claims
      .FirstOrDefault(c => string.Equals(c.Type, "Email", StringComparison.CurrentCultureIgnoreCase))?
      .Value;

    var fullName = jwtToken
      .Claims
      .FirstOrDefault(c => string.Equals(c.Type, "FullName"))?
      .Value;

    if (email is null || fullName is null)
    {
      throw new ArgumentException("Token inválido");
    }

    return new(email, fullName);
  }
}