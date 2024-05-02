namespace Koniradesk.server.DTOs;

public record StreamSharedDTO(string Cod, string Desktop, IAsyncEnumerable<byte[]> stream);