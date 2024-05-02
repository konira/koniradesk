using System.Net;
using System.Net.Sockets;

UdpClient udpClient = new UdpClient();
IPAddress ipAddress = new IPAddress(new byte[] { 236, 0, 0, 1 });
IPEndPoint ipEndPoint = new IPEndPoint(ipAddress, 2000);

var client = new HttpClient();
client.DefaultRequestVersion = HttpVersion.Version30;
client.DefaultVersionPolicy = HttpVersionPolicy.RequestVersionExact;

var resp = await client.GetAsync("https://localhost:5179/streaming/viewer", HttpCompletionOption.ResponseHeadersRead);

var body = await resp.Content.ReadAsStreamAsync();

try
{
    byte[] buffer = new byte[8192];
    int bytesRead;
    while ((bytesRead = await body.ReadAsync(buffer, 0, buffer.Length)) > 0)
    {
        await udpClient.SendAsync(buffer, bytesRead, ipEndPoint);
        await body.FlushAsync();

        Console.WriteLine(";" + bytesRead);
    }
}
catch (Exception e)
{
    Console.WriteLine(e.ToString());
}

Console.ReadLine();