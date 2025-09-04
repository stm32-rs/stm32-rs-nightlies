///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `MEM_MODE` reader - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000
pub type MEM_MODE_R = crate::FieldReader;
///Field `MEM_MODE` writer - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PA11_RMP` reader - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.
pub type PA11_RMP_R = crate::BitReader;
///Field `PA11_RMP` writer - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.
pub type PA11_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA12_RMP` reader - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.
pub type PA12_RMP_R = crate::BitReader;
///Field `PA12_RMP` writer - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.
pub type PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IR_POL` reader - IR output polarity selection
pub type IR_POL_R = crate::BitReader;
///Field `IR_POL` writer - IR output polarity selection
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IR_MOD` reader - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
pub type IR_MOD_R = crate::FieldReader;
///Field `IR_MOD` writer - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V).
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V).
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB6_FMP_R = crate::BitReader;
///Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB7_FMP_R = crate::BitReader;
///Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB8_FMP_R = crate::BitReader;
///Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB9_FMP_R = crate::BitReader;
///Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PA9_FMP_R = crate::BitReader;
///Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PA9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PA10_FMP_R = crate::BitReader;
///Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C_PA10_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3_FMP` reader - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C3_FMP_R = crate::BitReader;
///Field `I2C3_FMP` writer - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.
    #[inline(always)]
    pub fn pa11_rmp(&self) -> PA11_RMP_R {
        PA11_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.
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
    ///Bit 8 - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V).
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 24) & 1) != 0)
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
            .field("boosten", &self.boosten())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
            .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.
    #[inline(always)]
    pub fn pa11_rmp(&mut self) -> PA11_RMP_W<CFGR1rs> {
        PA11_RMP_W::new(self, 3)
    }
    ///Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.
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
    ///Bit 8 - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V).
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<CFGR1rs> {
        I2C_PA9_FMP_W::new(self, 22)
    }
    ///Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<CFGR1rs> {
        I2C_PA10_FMP_W::new(self, 23)
    }
    ///Bit 24 - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<CFGR1rs> {
        I2C3_FMP_W::new(self, 24)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
