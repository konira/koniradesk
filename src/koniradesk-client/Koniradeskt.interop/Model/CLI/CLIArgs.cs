namespace Koniradeskt.interop.Model.CLI
{
    public class CLIArgs
    {
        public string[] Args { get; set; }
        public string pipename { get; set; }
        public int port { get; set; }

        public CLIArgs(string[] args)
        {
            Args = args;
            if (args.Length == 0)
            {
                pipename = "koniradesk-01";
                port = 5179;
            }
            else
            {

                pipename = args[0];
                port = int.Parse(args[1]);
            }
        }
    }
}
