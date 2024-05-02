// commons.cpp: define o ponto de entrada para o aplicativo.
//

#include "commons.h"
#include "Encoders.h"
#include "EncodeH264.h"
#include "Enc"
using namespace std;

int main()
{    
	cout << "Hello CMake." << endl;
    Enconder::encodeRGBAtoH264();
	return 0;
}
