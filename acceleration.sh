# install GLX Gears, mesa GL and GLU libraries 
apt -y install mesa-utils

# install development tools
apt -y install build-essential automake pkg-config libtool ca-certificates git cmake subversion
                                                                                
# install required libraries                                                 
apt install libx11-dev libxext-dev xutils-dev libdrm-dev x11proto-xf86dri-dev libxfixes-dev
                                   
#  get source code                                                                                        
git clone https://github.com/robclark/libdri2                                   
git clone https://github.com/linux-sunxi/libump                                 
git clone https://github.com/linux-sunxi/sunxi-mali                             
git clone https://github.com/ssvb/xf86-video-fbturbo                            
git clone https://github.com/ptitSeb/glshim

# install mali driver
cd sunxi-mali                                                                   
git submodule init                                                              
git submodule update                                                            
git pull                                                                        
wget http://pastebin.com/raw.php?i=hHKVQfrh -O ./include/GLES2/gl2.h            
wget http://pastebin.com/raw.php?i=ShQXc6jy -O ./include/GLES2/gl2ext.h   
make config ABI=armhf VERSION=r3p0                                              
mkdir /usr/lib/mali                                                             
echo "/usr/lib/mali" > /etc/ld.so.conf.d/1-mali.conf                            
make -C include install                                                         
make -C lib/mali prefix=/usr libdir='$(prefix)/lib/mali/' install           
cd ..


# Step 1: build and install helper libraries                                    
                                                                                
cd libdri2                                                                      
autoreconf -i                                                                   
./configure --prefix=/usr                                                       
make                                                                            
make install                                                                    
cd ..                                                                           
                                                                                
cd libump                                                                       
autoreconf -i                                                                   
./configure --prefix=/usr                                                       
make                                                                            
make install                                                                    
cd ..                 

# Step 2: build video driver                              
                                                                                
cd xf86-video-fbturbo                                                           
autoreconf -i                                                                   
./configure --prefix=/usr                                                       
make                                                                            
make install                                                                    
cd ..     

# Step 3: build GL wrapper                                               
                                                                                
cd glshim                                                                       
cmake .                                                                         
make                                                                            
cp lib/libGL.so.1 /usr/lib/ # replace the software GL library with the wrapper
cd ..    