using System.Net;

namespace Koniradeskt.interop.Context
{
    public class KoniraDeskServerContext
    {
        public HttpClient Client { get; set; }

        public KoniraDeskServerContext(string baseUrl)
        {

            var client = new HttpClient();
            client.DefaultRequestVersion = HttpVersion.Version30;
            client.DefaultVersionPolicy = HttpVersionPolicy.RequestVersionExact;
            client.BaseAddress = new Uri(baseUrl);

            //var resp = await client.GetAsync("https://localhost:5179/streaming/viewer", HttpCompletionOption.ResponseHeadersRead);
            //var body = await resp.Content.ReadAsStreamAsync();
        }
    }
}
