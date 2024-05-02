using Grpc.Core;

namespace Koniradesk.server.Services
{
    public class KoniradeskService : Koniradesk.KoniradeskBase
    {
        private readonly ILogger<KoniradeskService> logger;
        public KoniradeskService(ILogger<KoniradeskService> _logger)
        {
            logger = _logger;
        }

        public override Task<GetKoniraDeskResponse> GetKoniraDesk(GetKoniraDeskRequest request, ServerCallContext context)
        {
            return base.GetKoniraDesk(request, context);
        }
    }
}
