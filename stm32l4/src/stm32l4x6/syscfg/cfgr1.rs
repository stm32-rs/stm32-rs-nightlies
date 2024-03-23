#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Field `FWDIS` reader - Firewall disable"]
pub type FWDIS_R = crate::BitReader;
#[doc = "Field `FWDIS` writer - Firewall disable"]
pub type FWDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub type BOOSTEN_R = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2C_PB6_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub type I2C_PB7_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub type I2C_PB8_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub type I2C_PB9_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation"]
pub type I2C1_FMP_R = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation"]
pub type I2C2_FMP_R = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation"]
pub type I2C3_FMP_R = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation"]
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE` reader - Floating Point Unit interrupts enable bits"]
pub type FPU_IE_R = crate::FieldReader;
#[doc = "Field `FPU_IE` writer - Floating Point Unit interrupts enable bits"]
pub type FPU_IE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Firewall disable"]
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Floating Point Unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall disable"]
    #[inline(always)]
    #[must_use]
    pub fn fwdis(&mut self) -> FWDIS_W<CFGR1rs> {
        FWDIS_W::new(self, 0)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<CFGR1rs> {
        I2C3_FMP_W::new(self, 22)
    }
    #[doc = "Bits 26:31 - Floating Point Unit interrupts enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<CFGR1rs> {
        FPU_IE_W::new(self, 26)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFGR1 to value 0x7c00_0001"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0x7c00_0001;
}
