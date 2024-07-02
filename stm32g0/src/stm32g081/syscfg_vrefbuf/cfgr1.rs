///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `MEM_MODE` reader - Memory mapping selection bits
pub type MEM_MODE_R = crate::FieldReader;
///Field `MEM_MODE` writer - Memory mapping selection bits
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit.
pub type PA11_PA12_RMP_R = crate::BitReader;
///Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit.
pub type PA11_PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IR_POL` reader - IR output polarity selection
pub type IR_POL_R = crate::BitReader;
///Field `IR_POL` writer - IR output polarity selection
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IR_MOD` reader - IR Modulation Envelope signal selection.
pub type IR_MOD_R = crate::FieldReader;
///Field `IR_MOD` writer - IR Modulation Envelope signal selection.
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1_STROBE` reader - Strobe signal bit for UCPD1
pub type UCPD1_STROBE_R = crate::BitReader;
///Field `UCPD1_STROBE` writer - Strobe signal bit for UCPD1
pub type UCPD1_STROBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD2_STROBE` reader - Strobe signal bit for UCPD2
pub type UCPD2_STROBE_R = crate::BitReader;
///Field `UCPD2_STROBE` writer - Strobe signal bit for UCPD2
pub type UCPD2_STROBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PBx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PBX_FMP_R = crate::FieldReader;
///Field `I2C_PBx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PBX_FMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PAx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PAX_FMP_R = crate::FieldReader;
///Field `I2C_PAx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PAX_FMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - PA11 and PA12 remapping bit.
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection.
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Strobe signal bit for UCPD1
    #[inline(always)]
    pub fn ucpd1_strobe(&self) -> UCPD1_STROBE_R {
        UCPD1_STROBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Strobe signal bit for UCPD2
    #[inline(always)]
    pub fn ucpd2_strobe(&self) -> UCPD2_STROBE_R {
        UCPD2_STROBE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 16:19 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pbx_fmp(&self) -> I2C_PBX_FMP_R {
        I2C_PBX_FMP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - FM+ driving capability activation for I2C1
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - FM+ driving capability activation for I2C2
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pax_fmp(&self) -> I2C_PAX_FMP_R {
        I2C_PAX_FMP_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("i2c_pax_fmp", &self.i2c_pax_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c_pbx_fmp", &self.i2c_pbx_fmp())
            .field("ucpd2_strobe", &self.ucpd2_strobe())
            .field("ucpd1_strobe", &self.ucpd1_strobe())
            .field("boosten", &self.boosten())
            .field("ir_mod", &self.ir_mod())
            .field("ir_pol", &self.ir_pol())
            .field("pa11_pa12_rmp", &self.pa11_pa12_rmp())
            .field("mem_mode", &self.mem_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 4 - PA11 and PA12 remapping bit.
    #[inline(always)]
    #[must_use]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W<CFGR1rs> {
        PA11_PA12_RMP_W::new(self, 4)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IR_POL_W<CFGR1rs> {
        IR_POL_W::new(self, 5)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection.
    #[inline(always)]
    #[must_use]
    pub fn ir_mod(&mut self) -> IR_MOD_W<CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - Strobe signal bit for UCPD1
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_strobe(&mut self) -> UCPD1_STROBE_W<CFGR1rs> {
        UCPD1_STROBE_W::new(self, 9)
    }
    ///Bit 10 - Strobe signal bit for UCPD2
    #[inline(always)]
    #[must_use]
    pub fn ucpd2_strobe(&mut self) -> UCPD2_STROBE_W<CFGR1rs> {
        UCPD2_STROBE_W::new(self, 10)
    }
    ///Bits 16:19 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    #[must_use]
    pub fn i2c_pbx_fmp(&mut self) -> I2C_PBX_FMP_W<CFGR1rs> {
        I2C_PBX_FMP_W::new(self, 16)
    }
    ///Bit 20 - FM+ driving capability activation for I2C1
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - FM+ driving capability activation for I2C2
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bits 22:23 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    #[must_use]
    pub fn i2c_pax_fmp(&mut self) -> I2C_PAX_FMP_W<CFGR1rs> {
        I2C_PAX_FMP_W::new(self, 22)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#SYSCFG_VREFBUF:CFGR1)*/
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
