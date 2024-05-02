using System.Net;
using System.Net.Sockets;
using System.Text;

namespace ConsoleClient.test;

public class UdpSocketServer
{
    private Socket socket;
    private byte[] buffer = new byte[1024];

    public UdpSocketServer(int port)
    {
        socket = new Socket(AddressFamily.InterNetwork, SocketType.Dgram, ProtocolType.Udp);
        socket.Bind(new IPEndPoint(IPAddress.Any, port));
    }

    public async Task Start()
    {
        Console.WriteLine("Servidor UDP iniciado...");
        BeginReceive();
    }

    private void BeginReceive()
    {
        socket.BeginReceive(buffer, 0, buffer.Length, SocketFlags.None, ReceiveCallback, null);
    }

    private void ReceiveCallback(IAsyncResult ar)
    {
        int received = socket.EndReceive(ar);
        byte[] data = new byte[received];
        Array.Copy(buffer, data, received);

        string message = Encoding.UTF8.GetString(data);
        Console.WriteLine($"Mensagem recebida: {message}");

        BeginReceive();
    }
}