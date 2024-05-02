namespace Koniradesk.server.Models.StreamingSharedModel
{
    public class StreamMonitor
    {
        public string cod { get; set; }
        public byte format { get; set; }
        public byte codec { get; set; }
        public Stream stream { get; set; }
    }
}
