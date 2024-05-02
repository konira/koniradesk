using Koniradesk.server;
using Koniradesk.server.Models.StreamingSharedModel;
using Koniradesk.server.Services;
using KoniraDesk.Server.Context;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Server.Kestrel.Core;
using System.Collections.Generic;
using System.Diagnostics;
using static System.Net.Mime.MediaTypeNames;
using System.IO;
using System.Net.Sockets;

var builder = WebApplication.CreateBuilder(args);
builder.WebHost.ConfigureKestrel(options =>
{
    options.ListenAnyIP(4443, listenOptions =>
    {
        listenOptions.Protocols = HttpProtocols.Http1AndHttp2AndHttp3;
        listenOptions.UseHttps();
    });
});
builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.TypeInfoResolverChain.Insert(0, AppJsonSerializerContext.Default);
});

builder.Services.AddSingleton<StreamingPool>();

builder.Services.AddGrpc();

var app = builder.Build();

app.MapGrpcService<GreeterService>();
app.MapGrpcService<KoniradeskService>();
app.MapGet("/", () => "Communication with gRPC endpoints must be made through a gRPC client. To learn how to create a client, visit: https://go.microsoft.com/fwlink/?linkid=2086909");
app.MapGet("/ping", async (CancellationToken token) => ping_stream.Ping(token));
app.MapPost("/streaming/shared", async ([FromServices] HttpContent context, [FromServices] StreamingPool pool, CancellationToken cancellation) =>
{   

    var type = context.Headers.GetValues("Content-Type");
    if (type.Contains("application/octet-stream"))
    {
        var encoding = context.Headers.GetValues("Content-Encoding");// video/webm;codecs="vp9"
        var format = context.Headers.GetValues("Content-Format");// webm
        var codec = context.Headers.GetValues("Content-Codec"); // vp9
    }

    using (Stream raw_stream = context.ReadAsStream())
    {
        using (Stream stream = new MemoryStream())
        {
            var count = 0;
            while (!cancellation.IsCancellationRequested)
            {
                count++;
                byte[] buffer = new byte[1];
                await raw_stream.ReadAsync(buffer, 0, 1, cancellation);
                if (count == 0)
                {
                    var first_frame = FrameMsg.Parser.ParseFrom(buffer);
                    pool.Pool.Add(new StreamMonitor
                    {
                        cod = first_frame.CodOrigin,
                        format = (byte)first_frame.Format,
                        codec = (byte)first_frame.Codec,
                        stream = stream
                    });
                }       
                
                var framemgs = FrameMsg.Parser.ParseFrom(buffer);
                await stream.WriteAsync(framemgs.Frame.ToByteArray(), 0, framemgs.Frame.Length);
            }
        }        
    }      
     
    return Results.Ok();
});

app.Run();

public static class ping_stream
{
    public static async IAsyncEnumerable<byte> Ping(CancellationToken token)
    {

        while (!token.IsCancellationRequested)
        {
            yield return 0;
            await Task.Delay(1000);
        }
    }
}