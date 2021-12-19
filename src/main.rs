use std::process::Command;
use std::env;
use std::io;
struct SpirvInfo{
    shaders : Vec<VkShaderInfo>,
    shader_location: String,
    sdk_location: String,
    compile_shader_count :usize
}
struct VkShaderInfo{
    shader_filename: String,
    object_filename: String,
    entry_name: String
}
fn get_input() -> String
{
    let mut buffer = String::from("\r\n"); 
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer){
        Ok(n)=>{
            println!("{}바이트 입력", n);     
        }
        Err(_e)=>{
            println!("{}",_e);
        }
    }
    return buffer;  
}
impl SpirvInfo
{
    fn get_info(&mut self)
    {
        let key = "VULKAN_SDK";
        match env::var(key) {
            Ok(val) => {
                println!("{} 환경 변수를 발견했습니다. 위치: {} 기본 SDK 경로로 지정합니다!", key, val);
                self.sdk_location = val;
            }
            Err(e) => println!("VULKAN_SDK 환경변수가 존재하지 않습니다! SDK 설치 여부를 확인 하세요! : {}", e)
        }
        {
            println!("셰이더 경로를 지정해주세요: ");
            let mut buffer = get_input();
            buffer.push('\\');
            self.shader_location  = String::from(buffer.replace("\\", "/").trim());      
        }

        {
            println!(" 컴파일할 셰이더의 개수는?: ");
            let buffer = get_input();
            let trimmed = buffer.trim();
            match trimmed.parse::<u32>() {
                Ok(i) =>{
                    self.compile_shader_count = i as usize;
                    println!("{}개 입력 확인 되었습니다!", self.compile_shader_count);
            },
                Err(..) => println!("정수를 입력해주세요!: {}", trimmed),
            };
        }

        for it in 0..self.compile_shader_count{
            
                println!("{}번째 셰이더 이름을 입력해주세요: ", it);
                let buffer_shader_name = get_input();

                println!("{}번째 셰이더 컴파일 후 이름을 입력해주세요: ", it);
                let buffer_object_name = get_input();
            
            self.shaders.push(VkShaderInfo{
                shader_filename: buffer_shader_name,
                object_filename: buffer_object_name,
                entry_name: String::from("main")
            });

        }
    }
    fn execute_cmd(&mut self)
    {
            
        self.sdk_location.push_str("\\Bin\\glslc.exe");
        self.sdk_location = self.sdk_location.replace("\\", "/");
        for it in 0..self.compile_shader_count{
            println!("{}번째 셰이더를 컴파일하고 있습니다!",it);
            let mut target_location = self.shader_location.clone();
            target_location.push_str(self.shaders[it].shader_filename.trim());

            let mut object_location = self.shader_location.clone();
            object_location.push_str(self.shaders[it].object_filename.trim());

            let mut entry_argument = String::from("-fentry-point=");
            entry_argument.push_str(self.shaders[it].entry_name.trim());

            Command::new(self.sdk_location.trim())
                .args([target_location.trim(),
                 entry_argument.trim(),
                 "-o", 
                 object_location.trim()])
                .spawn()
                .expect("명령어를 실행할 수 없습니다.");
        }
    }
}
fn main() {
    let mut basic_setting = SpirvInfo{
        shaders : Vec::new(),
        shader_location: String::from("D:/CodesAndProjects/Codes/Cpp_C/Graphics/vulkan_tutorial/vulkan_ex1/"),
        sdk_location:  String::from("C:/VulkanSDK/"),
        compile_shader_count: 2
    } ;
    basic_setting.get_info();
    basic_setting.execute_cmd();
}
