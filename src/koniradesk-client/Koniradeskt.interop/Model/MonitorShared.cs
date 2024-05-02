public class MonitorShared
{
    public required string Id { get; set; }
    public required string Name { get; set; }
    public required IAsyncEnumerable<byte[]> Frames { get; set; }
}