using Koniradeskt.interop.Model.CLI;
using Microsoft.AspNetCore.Server.Kestrel.Core;
using System.Text;

var builder = WebApplication.CreateSlimBuilder(args);
builder.WebHost.ConfigureKestrel(serverOptions =>
{
    args = args ?? Array.Empty<string>();
    var cli = new CLIArgs(args);
    if (OperatingSystem.IsWindows())
    {
        serverOptions.ListenNamedPipe(cli.pipename, listenOptions =>
        {
            listenOptions.Protocols = HttpProtocols.Http2;
            listenOptions.UseConnectionLogging();
        });
    }
    else
    {
        serverOptions.ListenUnixSocket(cli.pipename, listenOptions =>
        {
            listenOptions.Protocols = HttpProtocols.Http2;
            listenOptions.UseConnectionLogging();
        });
    }
    serverOptions.ListenLocalhost(cli.port);
});
builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.TypeInfoResolverChain.Insert(0, AppJsonSerializerContext.Default);
});

var app = builder.Build();

var remoteApi = app.MapGroup("/remote");

app.MapGet("/ping", async () => ping_stream.Ping());

app.Run();

public static class ping_stream
{
    public static async IAsyncEnumerable<byte[]> Ping()
    {

        while (true)
        {
            yield return Encoding.ASCII.GetBytes("pong");
            await Task.Delay(1000);
        }
    }
}