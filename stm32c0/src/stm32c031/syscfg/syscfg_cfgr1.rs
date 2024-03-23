#[doc = "Register `SYSCFG_CFGR1` reader"]
pub type R = crate::R<SYSCFG_CFGR1rs>;
#[doc = "Register `SYSCFG_CFGR1` writer"]
pub type W = crate::W<SYSCFG_CFGR1rs>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to for more details. x0: Main Flash memory"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to for more details. x0: Main Flash memory"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_RMP` reader - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
pub type PA11_RMP_R = crate::BitReader;
#[doc = "Field `PA11_RMP` writer - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
pub type PA11_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA12_RMP` reader - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
pub type PA12_RMP_R = crate::BitReader;
#[doc = "Field `PA12_RMP` writer - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
pub type PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IR_POL_R = crate::BitReader;
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_MOD` reader - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
pub type IR_MOD_R = crate::FieldReader;
#[doc = "Field `IR_MOD` writer - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB6_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB7_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB8_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB9_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers. With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C1_FMP_R = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers. With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PA9_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PA9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PA10_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PA10_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PC14_FMP` reader - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PC14_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PC14_FMP` writer - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
pub type I2C_PC14_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to for more details. x0: Main Flash memory"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
    #[inline(always)]
    pub fn pa11_rmp(&self) -> PA11_RMP_R {
        PA11_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
    #[inline(always)]
    pub fn pa12_rmp(&self) -> PA12_RMP_R {
        PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers. With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    pub fn i2c_pc14_fmp(&self) -> I2C_PC14_FMP_R {
        I2C_PC14_FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to for more details. x0: Main Flash memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<SYSCFG_CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
    #[inline(always)]
    #[must_use]
    pub fn pa11_rmp(&mut self) -> PA11_RMP_W<SYSCFG_CFGR1rs> {
        PA11_RMP_W::new(self, 3)
    }
    #[doc = "Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
    #[inline(always)]
    #[must_use]
    pub fn pa12_rmp(&mut self) -> PA12_RMP_W<SYSCFG_CFGR1rs> {
        PA12_RMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IR_POL_W<SYSCFG_CFGR1rs> {
        IR_POL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
    #[inline(always)]
    #[must_use]
    pub fn ir_mod(&mut self) -> IR_MOD_W<SYSCFG_CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    #[doc = "Bit 20 - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers. With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<SYSCFG_CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PA9_FMP_W::new(self, 22)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PA10_FMP_W::new(self, 23)
    }
    #[doc = "Bit 24 - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pc14_fmp(&mut self) -> I2C_PC14_FMP_W<SYSCFG_CFGR1rs> {
        I2C_PC14_FMP_W::new(self, 24)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_CFGR1rs;
impl crate::RegisterSpec for SYSCFG_CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr1::R`](R) reader structure"]
impl crate::Readable for SYSCFG_CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr1::W`](W) writer structure"]
impl crate::Writable for SYSCFG_CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR1 to value 0"]
impl crate::Resettable for SYSCFG_CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
