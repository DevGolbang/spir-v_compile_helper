use std::process::Command;

struct SpirvInfo{
    shaders : [VkShaderInfo; 2],
    shader_location: String,
    sdk_location: String,
    sdk_version: String
}
struct VkShaderInfo{
    shader_filename: String,
    object_filename: String,
    entry_name: String
}

impl SpirvInfo
{

    fn execute_cmd(&mut self)
    {
            
        self.sdk_location.push_str(self.sdk_version.as_str());
        self.sdk_location.push_str("/Bin/glslc.exe");
        for it in 0..2{
            println!("{}번째 셰이더를 컴파일하고 있습니다!",it);
            let mut target_location = self.shader_location.clone();
            target_location.push_str(self.shaders[it].shader_filename.as_str());
    
            let mut object_location = self.shader_location.clone();
            object_location.push_str(self.shaders[it].object_filename.as_str());

            let mut entry_argument = String::from("-fentry-point=");
            entry_argument.push_str(self.shaders[it].entry_name.as_str());

            Command::new(self.sdk_location.as_str())
                .args([target_location.as_str(),
                 entry_argument.as_str(),
                 "-o", 
                 object_location.as_str(),])
                .spawn()
                .expect("명령어를 실행할 수 없습니다.");
        }



    }
}
fn main() {
    let mut basic_setting = SpirvInfo{
        shaders : [
            VkShaderInfo{
                shader_filename: String::from("shader.vert"),
                object_filename: String::from("main_vert.spv"),
                entry_name: String::from("main"),
            },
            VkShaderInfo{
                shader_filename: String::from("shader.frag"),
                object_filename: String::from("main_frag.spv"),
                entry_name: String::from("main"),
            },
        ],
        shader_location: String::from("D:/CodesAndProjects/Codes/Cpp_C/Graphics/vulkan_tutorial/vulkan_ex1/"),
        sdk_location: String::from("C:/VulkanSDK/"),
        sdk_version: String::from("1.2.189.2")
    } ;
    basic_setting.execute_cmd();

}
