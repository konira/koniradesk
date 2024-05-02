using System.Text.Json.Serialization;

[JsonSerializable(typeof(MonitorShared))]
[JsonSerializable(typeof(byte[]))]
[JsonSerializable(typeof(IAsyncEnumerable<byte[]>))]
[JsonSerializable(typeof(IEnumerable<byte[]>))]
internal partial class AppJsonSerializerContext : JsonSerializerContext
{

}
