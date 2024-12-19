///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `MEM_MODE` reader - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
pub type MEM_MODE_R = crate::FieldReader;
///Field `MEM_MODE` writer - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `PA11_RMP` reader - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\]
bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
pub type PA11_RMP_R = crate::BitReader;
/**Field `PA11_RMP` writer - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\]
bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
pub type PA11_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PA12_RMP` reader - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\]
bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
pub type PA12_RMP_R = crate::BitReader;
/**Field `PA12_RMP` writer - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\]
bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
pub type PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IR_POL` reader - IR output polarity selection
pub type IR_POL_R = crate::BitReader;
///Field `IR_POL` writer - IR output polarity selection
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IR_MOD` reader - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
pub type IR_MOD_R = crate::FieldReader;
///Field `IR_MOD` writer - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
pub type I2C_PB6_FMP_R = crate::BitReader;
///Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
pub type I2C_PB7_FMP_R = crate::BitReader;
///Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB8_FMP_R = crate::BitReader;
///Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB9_FMP_R = crate::BitReader;
///Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_FMP` reader - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
pub type I2C_PA9_FMP_R = crate::BitReader;
///Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
pub type I2C_PA9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
pub type I2C_PA10_FMP_R = crate::BitReader;
///Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
pub type I2C_PA10_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PC14_FMP` reader - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PC14_FMP_R = crate::BitReader;
///Field `I2C_PC14_FMP` writer - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PC14_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    /**Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\]
    bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
    #[inline(always)]
    pub fn pa11_rmp(&self) -> PA11_RMP_R {
        PA11_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    /**Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\]
    bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
    #[inline(always)]
    pub fn pa12_rmp(&self) -> PA12_RMP_R {
        PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pc14_fmp(&self) -> I2C_PC14_FMP_R {
        I2C_PC14_FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("mem_mode", &self.mem_mode())
            .field("pa11_rmp", &self.pa11_rmp())
            .field("pa12_rmp", &self.pa12_rmp())
            .field("ir_pol", &self.ir_pol())
            .field("ir_mod", &self.ir_mod())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
            .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
            .field("i2c_pc14_fmp", &self.i2c_pc14_fmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    /**Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\]
    bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
    #[inline(always)]
    pub fn pa11_rmp(&mut self) -> PA11_RMP_W<CFGR1rs> {
        PA11_RMP_W::new(self, 3)
    }
    /**Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\]
    bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.*/
    #[inline(always)]
    pub fn pa12_rmp(&mut self) -> PA12_RMP_W<CFGR1rs> {
        PA12_RMP_W::new(self, 4)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W<CFGR1rs> {
        IR_POL_W::new(self, 5)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
    #[inline(always)]
    pub fn ir_mod(&mut self) -> IR_MOD_W<CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    ///Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
    #[inline(always)]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<CFGR1rs> {
        I2C_PA9_FMP_W::new(self, 22)
    }
    ///Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
    #[inline(always)]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<CFGR1rs> {
        I2C_PA10_FMP_W::new(self, 23)
    }
    ///Bit 24 - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pc14_fmp(&mut self) -> I2C_PC14_FMP_W<CFGR1rs> {
        I2C_PC14_FMP_W::new(self, 24)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
