#[doc = "Register `DFSDM_CHCFG1R1` reader"]
pub type R = crate::R<DFSDM_CHCFG1R1rs>;
#[doc = "Register `DFSDM_CHCFG1R1` writer"]
pub type W = crate::W<DFSDM_CHCFG1R1rs>;
#[doc = "Field `SITP` reader - Serial interface type for channel 1"]
pub type SITP_R = crate::FieldReader;
#[doc = "Field `SITP` writer - Serial interface type for channel 1"]
pub type SITP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPICKSEL` reader - SPI clock select for channel 1"]
pub type SPICKSEL_R = crate::FieldReader;
#[doc = "Field `SPICKSEL` writer - SPI clock select for channel 1"]
pub type SPICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCDEN` reader - Short-circuit detector enable on channel 1"]
pub type SCDEN_R = crate::BitReader;
#[doc = "Field `SCDEN` writer - Short-circuit detector enable on channel 1"]
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABEN` reader - Clock absence detector enable on channel 1"]
pub type CKABEN_R = crate::BitReader;
#[doc = "Field `CKABEN` writer - Clock absence detector enable on channel 1"]
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN` reader - Channel 1 enable"]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel 1 enable"]
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHINSEL` reader - Channel inputs selection"]
pub type CHINSEL_R = crate::BitReader;
#[doc = "Field `CHINSEL` writer - Channel inputs selection"]
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATMPX` reader - Input data multiplexer for channel 1"]
pub type DATMPX_R = crate::FieldReader;
#[doc = "Field `DATMPX` writer - Input data multiplexer for channel 1"]
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATPACK` reader - Data packing mode in DFSDM_CHDATINyR register"]
pub type DATPACK_R = crate::FieldReader;
#[doc = "Field `DATPACK` writer - Data packing mode in DFSDM_CHDATINyR register"]
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKOUTDIV` reader - Output serial clock divider"]
pub type CKOUTDIV_R = crate::FieldReader;
#[doc = "Field `CKOUTDIV` writer - Output serial clock divider"]
pub type CKOUTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKOUTSRC` reader - Output serial clock source selection"]
pub type CKOUTSRC_R = crate::BitReader;
#[doc = "Field `CKOUTSRC` writer - Output serial clock source selection"]
pub type CKOUTSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMEN` reader - Global enable for DFSDM interface"]
pub type DFSDMEN_R = crate::BitReader;
#[doc = "Field `DFSDMEN` writer - Global enable for DFSDM interface"]
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Serial interface type for channel 1"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel 1"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel 1"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel 1"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel inputs selection"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel 1"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Output serial clock divider"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Output serial clock source selection"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial interface type for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<DFSDM_CHCFG1R1rs> {
        SITP_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<DFSDM_CHCFG1R1rs> {
        SPICKSEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<DFSDM_CHCFG1R1rs> {
        SCDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<DFSDM_CHCFG1R1rs> {
        CKABEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<DFSDM_CHCFG1R1rs> {
        CHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel inputs selection"]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<DFSDM_CHCFG1R1rs> {
        CHINSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<DFSDM_CHCFG1R1rs> {
        DATMPX_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register"]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<DFSDM_CHCFG1R1rs> {
        DATPACK_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - Output serial clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<DFSDM_CHCFG1R1rs> {
        CKOUTDIV_W::new(self, 16)
    }
    #[doc = "Bit 30 - Output serial clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<DFSDM_CHCFG1R1rs> {
        CKOUTSRC_W::new(self, 30)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<DFSDM_CHCFG1R1rs> {
        DFSDMEN_W::new(self, 31)
    }
}
#[doc = "DFSDM channel configuration 1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg1r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg1r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_CHCFG1R1rs;
impl crate::RegisterSpec for DFSDM_CHCFG1R1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_chcfg1r1::R`](R) reader structure"]
impl crate::Readable for DFSDM_CHCFG1R1rs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_chcfg1r1::W`](W) writer structure"]
impl crate::Writable for DFSDM_CHCFG1R1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_CHCFG1R1 to value 0"]
impl crate::Resettable for DFSDM_CHCFG1R1rs {
    const RESET_VALUE: u32 = 0;
}
