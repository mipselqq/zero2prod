public class MGSamples {
  public static JsonNode sendSimpleMessage() throws UnirestException {
    String apiKey = System.getenv("API_KEY");
        if (apiKey == null) {
            apiKey = "906330556af8a09b91fec29b75be65e2-556e0aa9-694885ae";
        }

    HttpResponse<JsonNode> request = Unirest.post("https://api.mailgun.net/v3/sandboxeddff7d118394e24bebb8990b9ff352d.mailgun.org/messages")
      .basicAuth("api", apiKey)
      .queryString("from", "Mailgun Sandbox <postmaster@sandboxeddff7d118394e24bebb8990b9ff352d.mailgun.org>")
      .queryString("to", "Main <lardodafyo@necub.com>")
      .queryString("subject", "Hello Main")
      .queryString("text", "Congratulations Main, you just sent an email with Mailgun! You are truly awesome!")
      .asJson();
    return request.getBody();
  }
  }
}
