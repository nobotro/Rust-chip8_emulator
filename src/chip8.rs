


const FONTS:[u8;80] =[0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90,
0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90,
0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0,
0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0,
0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];

const f2:[u8;3]=[1,2,3];


    struct OP{
       name:u16,
       handler:fn(&mut CPU)
    }



pub struct CPU{
    rom_load_address:u16,
    pub display:[u8; 2048],
    ram: [u8; 4096],
    v_registers: [u16; 16],
    i_register:u16,
    delay_register:u16,
    sound_register:u16 , 
    pc_register:u16,
    stack:[u16; 16],
    stack_index:usize, //store index of stack
    draw_flag:bool,
    pub keyboard:[u8;16],
    opcode:u16,
    active:bool,
    opcodes_mapping:[OP;33]
    
}

impl CPU {

    
    pub fn new() -> CPU {
        
        let mut cpu = CPU
        {
            rom_load_address:0x200,
            display:[0; 2048],
            ram:[0;4096],
            v_registers:[0;16],
            i_register:0,
            delay_register:0,
            sound_register:0,
            pc_register:0x200,
            stack:[0;16],
            stack_index:0,
            draw_flag:false,
            keyboard:[0;16],
            opcode:0,
            active:false,
            opcodes_mapping:[

                    OP{name: 0x00, handler:CPU::_00 }, OP{name: 0x0E, handler:CPU::_0E }, OP{name: 0x1, handler:CPU::_1 }, OP{name: 0x2, handler:CPU::_2 },
                    OP{name: 0x3, handler:CPU::_3 }, OP{name: 0x4, handler:CPU::_4 }, OP{name: 0x5, handler:CPU::_5 }, OP{name: 0x6, handler:CPU::_6 }, 
                    OP{name: 0x7, handler:CPU::_7 }, OP{name: 0x80,handler:CPU::_80 }, OP{name: 0x81, handler:CPU::_81 }, OP{name: 0x82, handler:CPU::_82 },
                    OP{name: 0x83, handler:CPU::_83 },OP{name: 0x84, handler:CPU::_84 }, OP{name: 0x85, handler:CPU::_85 }, OP{name: 0x86, handler:CPU::_86 }, 
                    OP{name: 0x87, handler:CPU::_87 },OP{name: 0x8E, handler:CPU::_8E }, OP{name: 0x9, handler:CPU::_9 }, OP{name:0xA, handler:CPU::_A }, 
                    OP{name: 0xB, handler:CPU::_B }, OP{name: 0xC, handler:CPU::_C }, OP{name:0xD, handler:CPU::_D }, OP{name: 0xEE, handler:CPU::_EE }, 
                    OP{name: 0xE1, handler:CPU::_E1 }, OP{name: 0xF07, handler:CPU::_F07 }, OP{name: 0xF15, handler:CPU::_F15 },
                    OP{name: 0x18, handler:CPU::_F18 },OP{name: 0xF1E, handler:CPU::_F1E }, OP{name: 0xF29, handler:CPU::_F29 }, OP{name: 0xF33, handler:CPU::_F33 }, 
                    OP{name: 0xF55, handler:CPU::_F55 }, OP{name:0xF65, handler:CPU::_F65 }
            ]       

           
             
            


        };

        cpu.init_fonts();
        cpu
    }
  
    
    pub fn run_loop(&mut self)
    {   if self.delay_register>0 {
		self.delay_register -= 1;
	}   

         self.opcode = u16::from(self.ram[self.pc_register as usize]) << 8 | self.ram[(self.pc_register + 1) as usize] as u16;
        self.pc_register = self.pc_register + 2;
         self.execop(self.opcode);
        
        
    }

    pub fn load_rom(&mut self,rom:&[u8]){       
        for i in 0..rom.len()
        {
                self.ram[i + self.rom_load_address as usize]= rom[i]

        }
 
    }


    fn execop(&mut self,opcode:u16){

        let f = (opcode & 0xF000) >> 12;
        if f == 8 || f == 0 || f == 14{

            let l:u16 = opcode & 0x000F;
            self.call_function( f<<4|l);

        } else if f == 15{

            let l:u16 = opcode & 0x000F;
            let ll:u16 = (opcode & 0x00F0) >> 4;
            self.call_function(((f<<4 |ll)<<4)|l);

        } else {

            self.call_function(f);

        }

    }

    


    fn call_function(&mut self,name:u16) {
            
        for i in 0..33{
            if self.opcodes_mapping[i].name == name {
        
                (self.opcodes_mapping[i].handler)(self); 
                break;
            }
        }

    }

    fn clear_screen(&mut self){

        
        for mut i in 0..self.display.len()
        {
            self.display[i as usize] = 0;
        }        

    }

    fn rand_number(&mut self) ->u32{

        let mut lfsr = 0b11010011; //for pseudo random number
        let bit: u32 = (lfsr>>0 ^ lfsr >>5 ^ lfsr >> 3)&1;
        lfsr = (lfsr >> 1) | (bit << 7);
        return lfsr;
    }


    fn init_fonts(&mut self){
        for i in 0..0x50 {
            self.ram[i as usize] = FONTS[i as usize];
        }
    }


    fn _00(&mut self) {

        // 00E0 - CLS Clear the display.
            self.clear_screen();
            self.draw_flag = true;
        }
    
    fn _0E(&mut self) {
        /*``
            00EE - RET
            Return from a subroutine.
            The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.'''
            */
        self.pc_register = self.stack[self.stack_index];
        self.stack_index -=1;
    }
    
    fn _1(&mut self) {
        /* 1nnn - JP addr
         jump to location nnn.
         The interpreter sets the program counter to nnn.
         */
    
        self.pc_register = self.opcode & 0x0FFF;
    
    }

    fn _2(&mut self) {
        /*2nnn - CALL addr
         Call subroutine at nnn.
         The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
         */
    
    //    self.stack.append(self.pc_register)
        self.stack_index+=1;
        self.stack[self.stack_index] = self.pc_register;
        self.pc_register = self.opcode & 0x0FFF;
    
    }
    
    fn _3(&mut self) {
        /*3xkk - SE Vx, byte
         Skip next instruction if Vx = kk.
         The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
         */
    
        let x:u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk:u8 = (self.opcode & 0x00FF ) as u8;
    
        if self.v_registers[x as usize] == kk as u16
        {
            self.pc_register += 2;
        }
            
    
    }

    fn _4(&mut self) {
        /*3xkk - SE Vx, byte
         Skip next instruction if Vx = kk.
         The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
         */
    
        let x:u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk:u8 = (self.opcode & 0x00FF ) as u8;
    
        if self.v_registers[x as usize] != kk as u16
        {
            self.pc_register += 2;
        }
            
    
    }

    fn _5(&mut self) {

        /*3xkk - SE Vx, byte
         Skip next instruction if Vx = kk.
         The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y= ((self.opcode & 0x00F0) >> 4) as usize;
	    if self.v_registers[x] == self.v_registers[y]
        {
            self.pc_register += 2;
        }
		       
    }

    fn _6(&mut self) {

        /*6xkk - LD Vx, byte
         Set Vx = kk.
         The interpreter puts the value kk into register Vx.*/
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = self.opcode & 0x00FF;
        self.v_registers[x] = kk;
    }

    fn _7(&mut self) {
        /*
         7xkk - ADD Vx, byte
         Set Vx = Vx + kk.
         Adds the value kk to the value of register Vx, then stores the result in Vx.
         */
        let x = (self.opcode & 0x0F00) >> 8;
        let kk = self.opcode & 0x00FF;
        self.v_registers[x as usize] += kk;
    
    }

    fn _80(&mut self) {
        /*8xy0 - LD Vx, Vy
         Set Vx = Vy.
         Stores the value of register Vy in register Vx.*/
    
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        self.v_registers[x as usize] = self.v_registers[y as usize];
    
    }

    fn _81(&mut self) {
        /*8xy1 - OR Vx, Vy
         Set Vx = Vx OR Vy.
         Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
         A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.v_registers[x] = self.v_registers[x] | self.v_registers[y];
    
    }
    
    fn _82(&mut self) {
        /*8xy2 - AND Vx, Vy
         Set Vx = Vx AND Vy.
         Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
         A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.v_registers[x] = self.v_registers[x] & self.v_registers[y];
    }

    fn _83(&mut self) {
        /*8xy3 - XOR Vx, Vy
         Set Vx = Vx XOR Vy.
         Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
         An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same,
         then the corresponding bit in the result is set to 1. Otherwise, it is 0.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.v_registers[x] = self.v_registers[x] ^ self.v_registers[y];
    
    }

    fn _84(&mut self) {
        /*8xy4 - ADD Vx, Vy
         Set Vx = Vx + Vy, set VF = carry.
         The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
         VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.v_registers[x] + self.v_registers[y] > 255 {
            self.v_registers[15] = 1;
        } else {
            self.v_registers[15] = 0;
        }
    
        self.v_registers[x] = self.v_registers[x] + self.v_registers[y];
    }

    fn _85(&mut self) {

        /*8xy5 - SUB Vx, Vy
         Set Vx = Vx - Vy, set VF = NOT borrow.
         If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
    
        if self.v_registers[x] > self.v_registers[y] {
            self.v_registers[15] = 1;
        } else {
            self.v_registers[15] = 0;
        }
        
    

        self.v_registers[x] = {
            if (self.v_registers[x] as i16 - self.v_registers[y] as i16) < 0{
               { 0}
            }
            else {
                {self.v_registers[x] - self.v_registers[y]}
            }
        }
         
    }

    fn _86(&mut self) {
        /*8xy6 - SHR Vx {, Vy} Set Vx = Vx SHR 1.
         If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.v_registers[15] = self.v_registers[x] & 1;
    
        self.v_registers[x] = self.v_registers[x] >> 1;
     
    }

    fn _87(&mut self) {
        /*8xy7 - SUBN Vx, Vy
         Set Vx = Vy - Vx, set VF = NOT borrow.
         If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
    
        if self.v_registers[y] > self.v_registers[x] {
            self.v_registers[15] = 1;
        } else {
            self.v_registers[15] = 0;
        }
    
        self.v_registers[x] = self.v_registers[y] - self.v_registers[x];
     
    }


    fn _8E(&mut self){
        /*8xyE - SHL Vx {, Vy}
        Set Vx = Vx SHL 1.
        If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
        */

        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        self.v_registers[15] = self.v_registers[x] >> 7;

        self.v_registers[x] = self.v_registers[x] << 1;
 
    }


    fn _9(&mut self) {
        /*9xy0 - SNE Vx, Vy
         Skip next instruction if Vx != Vy.
         The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.*/
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
    
        if self.v_registers[x] != self.v_registers[y] {
            self.pc_register += 2;
        }
    
    }

    fn _A(&mut self) {
        /*Annn - LD I, addr
         Set I = nnn.
         The value of register I is set to nnn.*/
    
        let nnn = self.opcode & 0x0FFF;
        self.i_register = nnn;
    
    }

    fn _B(&mut self) {
        /*Bnnn - JP V0, addr
         Jump to location nnn + V0.
         The program counter is set to nnn plus the value of V0.*/
    
        let nnn = self.opcode & 0x0FFF;
        self.pc_register = nnn + self.v_registers[0];
    }

    fn _C(&mut self) {
        /*Cxkk - RND Vx, byte
         Set Vx = random byte AND kk.
         The interpreter generates a random number from 0 to 255,
         which is then ANDed with the value kk.
         The results are stored in Vx. See instruction 8xy2 for more information on AND.*/
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = self.opcode & 0x00FF;
        let randn = self.rand_number();
    
        self.v_registers[x] = kk & randn as u16;
     }

    fn _EE(&mut self) {
        /*'''
    
         Ex9E - SKP Vx
         Skip next instruction if key with the value of Vx is pressed.
         Checks the keyboard, and if the key corresponding to the value of Vx is
         currently in the down position, PC is increased by 2.'''*/
         
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
    
        if self.keyboard[(self.v_registers[x] & 0xf) as usize] == 1 {
            self.pc_register += 2;
           
        }

    }

    fn _E1(&mut self) {
        /*'''ExA1 - SKNP Vx
         Skip next instruction if key with the value of Vx is not pressed.
         Checks the keyboard, and
         if the key corresponding to the value of Vx is currently in the up position,
         PC is increased by 2.
         '''*/
         
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
    
        if self.keyboard[(self.v_registers[x] & 0xf) as usize] == 0 {
    
            self.pc_register += 2;
            
        }
    
    }

    fn _F07(&mut self) {
        /*'''
         Fx07 - LD Vx, DT
         Set Vx = delay timer value.
         The value of DT is placed into Vx'''*/
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
    
        self.v_registers[x] = self.delay_register;
    }

    fn _F15(&mut self) {
        /*'''
         Set delay timer = Vx.
    
         DT is set equal to the value of Vx.
         '''
         */
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.delay_register = self.v_registers[x];
    
    }

    fn _F18(&mut self){

        /*'''
         Set sound timer = Vx.
    
         ST is set equal to the value of Vx.
    
         '''*/
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.sound_register = self.v_registers[x];
    }

    fn _F1E(&mut self) {
        /*'''
         Set I = I + Vx.
    
         The values of I and Vx are added, and the results are stored in I.'''*/
    
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.i_register = self.i_register + self.v_registers[x];
    
    }

    fn _F29(&mut self) {

        /*'''
    
         Set I = location of sprite for digit Vx.
    
         The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
         See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    
         '''*/
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.i_register = self.v_registers[x] * 5;
    
    }

    fn _F33(&mut self) {

        /*'''Store BCD representation of Vx in memory locations I, I+1, and I+2.
    
         The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I
         , the tens digit at location I+1, and the ones digit at location I+2.
         '''*/
    
        let x = self.v_registers[((self.opcode & 0x0F00) >> 8) as usize] as u16;
    
    
 
        let hundred = x / 100;
        let ten = (x - (hundred * 100)) / 10;
        let one = x - (hundred * 100) - (ten * 10);
        self.ram[self.i_register as usize] = hundred as u8;
        self.ram[(self.i_register + 1) as usize] = ten as u8;
        self.ram[(self.i_register + 2 )as usize] = one as u8;
    
    }

    fn _F55(&mut self) {

        /*'''
         Store registers V0 through Vx in memory starting at location I.
    
         The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    
         '''*/
    
        let x = (self.opcode & 0x0F00) >> 8;
        for i in 0..=x
        {
            self.ram[(self.i_register + i) as usize] = self.v_registers[i as usize] as u8;
        }
       
    
    }


    fn _F65(&mut self) {

        /* '''
    
         Read registers V0 through Vx from memory starting at location I.
    
         The interpreter reads values from memory starting at location I into registers V0 through Vx.
         '''*/
    
        let x = (self.opcode & 0x0F00) >> 8;
        for i in 0..=x
        {
            self.v_registers[i as usize] = self.ram[(self.i_register + i) as usize] as u16;
        }
      
    }


    fn _D(&mut self) {
        /*Dxyn - DRW Vx, Vy, nibble
         Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    
         The interpreter reads n bytes from memory, starting at the address stored in I.
         These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
         Sprites are XORed onto the existing screen.
         If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
         If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
         See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.'''*/
        // for(int i=0;i<15;i++){
        //     	*LEDS=i;
        // 			for(int j=0;j<19000000;j++){}
        // 		}
    
        
        let vx = self.v_registers[((self.opcode & 0x0F00) >> 8) as usize];
        let vy = self.v_registers[((self.opcode & 0x00F0) >> 4) as usize];
        let n = self.opcode & 0x000F;
        self.v_registers[0xF] = 0;
        for y in 0..n{
            let pixel = self.ram[(self.i_register + y) as usize];
            for x in 0..8{
                if (pixel & (0x80 >> x))!=0 && x + vx + ((y + vy) * 64) < 2048 {
                    
    
    
                    if self.display[(x + vx + ((y + vy) * 64)) as usize]!=0
                    {
    
                        self.v_registers[0xF] = 1;
                    }
                
                    self.display[(x + vx + ((y + vy) * 64)) as usize]^=1;
    
                }
    
            }
    
        }
    
        self.draw_flag = true;
    
    }
    

}
