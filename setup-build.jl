ENV["LIBCLANG_PATH"] = "C:/Program Files/Microsoft Visual Studio/2022/Preview/VC/Tools/Llvm/x64/bin"
ENV["VCPKG_ROOT"]= "C:/vcpkg"
ENV["VCPKGRS_DYNAMIC"]=1
ENV["VCPKG_DEFAULT_TRIPLET"]="x64-windows"
ENV["FFMPEG_DIR"]="C:/vcpkg/packages/ffmpeg_x64-windows"
#ENV["FFMPEG_INCLUDE"]="C:/vcpkg/packages/ffmpeg_x64-windows/include"
#-DCMAKE_TOOLCHAIN_FILE=C:/vcpkg/scripts/buildsystems/vcpkg.cmake
#install vcpkg and libs
ENV["PATH"] = ENV["VCPKG_ROOT"] * ";" * "C:\\vcpkg\\packages\\ffmpeg_x64-windows\\tools\\ffmpeg" *";"* ENV["PATH"]  
if Sys.iswindows()
    if !isdir(ENV["VCPKG_ROOT"])
        run(`powershell.exe git clone https://github.com/Microsoft/vcpkg.git  $env:VCPKG_ROOT`, wait=true)    
        ENV["PATH"] = ENV["VCPKG_ROOT"] * ";" * ENV["PATH"] 
        run(`powershell.exe bootstrap-vcpkg.bat`, wait=true,)
        run(`powershell.exe vcpkg.exe integrate install`, wait=true,)  
    end    
    #if !isdir(ENV["VCPKG_ROOT"]*"/installed")
    #    run(`powershell.exe vcpkg.exe install "opencv[contrib,nonfree,ade]"`, wait=true)
    #end         
end
# run(`cargo build`, wait=true)
run(Cmd(["Code.exe","."]))