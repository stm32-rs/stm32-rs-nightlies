#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit."]
pub type PA11_PA12_RMP_R = crate::BitReader;
#[doc = "Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit."]
pub type PA11_PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IR_POL_R = crate::BitReader;
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_MOD` reader - IR Modulation Envelope signal selection."]
pub type IR_MOD_R = crate::FieldReader;
#[doc = "Field `IR_MOD` writer - IR Modulation Envelope signal selection."]
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub type BOOSTEN_R = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1_STROBE` reader - Strobe signal bit for UCPD1"]
pub type UCPD1_STROBE_R = crate::BitReader;
#[doc = "Field `UCPD1_STROBE` writer - Strobe signal bit for UCPD1"]
pub type UCPD1_STROBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD2_STROBE` reader - Strobe signal bit for UCPD2"]
pub type UCPD2_STROBE_R = crate::BitReader;
#[doc = "Field `UCPD2_STROBE` writer - Strobe signal bit for UCPD2"]
pub type UCPD2_STROBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PBx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PBX_FMP_R = crate::FieldReader;
#[doc = "Field `I2C_PBx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PBX_FMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1"]
pub type I2C1_FMP_R = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2"]
pub type I2C2_FMP_R = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PAx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PAX_FMP_R = crate::FieldReader;
#[doc = "Field `I2C_PAx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PAX_FMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit."]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection."]
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Strobe signal bit for UCPD1"]
    #[inline(always)]
    pub fn ucpd1_strobe(&self) -> UCPD1_STROBE_R {
        UCPD1_STROBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Strobe signal bit for UCPD2"]
    #[inline(always)]
    pub fn ucpd2_strobe(&self) -> UCPD2_STROBE_R {
        UCPD2_STROBE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pbx_fmp(&self) -> I2C_PBX_FMP_R {
        I2C_PBX_FMP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pax_fmp(&self) -> I2C_PAX_FMP_R {
        I2C_PAX_FMP_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit."]
    #[inline(always)]
    #[must_use]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W<CFGR1rs> {
        PA11_PA12_RMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IR_POL_W<CFGR1rs> {
        IR_POL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection."]
    #[inline(always)]
    #[must_use]
    pub fn ir_mod(&mut self) -> IR_MOD_W<CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Strobe signal bit for UCPD1"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_strobe(&mut self) -> UCPD1_STROBE_W<CFGR1rs> {
        UCPD1_STROBE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Strobe signal bit for UCPD2"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2_strobe(&mut self) -> UCPD2_STROBE_W<CFGR1rs> {
        UCPD2_STROBE_W::new(self, 10)
    }
    #[doc = "Bits 16:19 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pbx_fmp(&mut self) -> I2C_PBX_FMP_W<CFGR1rs> {
        I2C_PBX_FMP_W::new(self, 16)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pax_fmp(&mut self) -> I2C_PAX_FMP_W<CFGR1rs> {
        I2C_PAX_FMP_W::new(self, 22)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
