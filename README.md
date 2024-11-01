# automatic-octo-eureka
Hardware Virtualization (First Lab)

[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you make a project for the first time, I recommand you to use the ```cargo new your_project``` command. Your cargo command cannot work automatically in your current file architecture. It could eventually, if you specify your real .rs locations in your Cargo.toml file, but I really recommand to do a project with a src folder (cf ```cargo new your_project``` command), or you may be lost with all your files.
Consider subdividing your project into separate modules.
You can only use the I/O registers of port B here (and not the C port for example).
It would be nice to have something to prevent modifying the register in an incoherent way. For example, if I do ``` modify_reg(reg: 0x25 as *mut u8, pin: 40, set: true);```, it won't bug during the compilation, but it will generate a problem on your hardware.