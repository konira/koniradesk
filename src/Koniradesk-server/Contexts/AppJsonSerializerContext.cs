using Koniradesk.server.DTOs;
using System.Text.Json.Serialization;

namespace KoniraDesk.Server.Context;

[JsonSerializable(typeof(byte))]
[JsonSerializable(typeof(byte[]))]
[JsonSerializable(typeof(IAsyncEnumerable<byte[]>))]
[JsonSerializable(typeof(IEnumerable<byte[]>))]
[JsonSerializable(typeof(StreamSharedDTO))]
internal partial class AppJsonSerializerContext : JsonSerializerContext
{

}
