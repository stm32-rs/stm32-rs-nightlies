#[doc = "Register `FDCAN_TTTMC` reader"]
pub type R = crate::R<FDCAN_TTTMCrs>;
#[doc = "Register `FDCAN_TTTMC` writer"]
pub type W = crate::W<FDCAN_TTTMCrs>;
#[doc = "Field `TMSA` reader - TMSA"]
pub type TMSA_R = crate::FieldReader<u16>;
#[doc = "Field `TMSA` writer - TMSA"]
pub type TMSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TME` reader - TME"]
pub type TME_R = crate::FieldReader;
#[doc = "Field `TME` writer - TME"]
pub type TME_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 2:15 - TMSA"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - TMSA"]
    #[inline(always)]
    #[must_use]
    pub fn tmsa(&mut self) -> TMSA_W<FDCAN_TTTMCrs> {
        TMSA_W::new(self, 2)
    }
    #[doc = "Bits 16:22 - TME"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TME_W<FDCAN_TTTMCrs> {
        TME_W::new(self, 16)
    }
}
#[doc = "FDCAN TT trigger memory configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTTMCrs;
impl crate::RegisterSpec for FDCAN_TTTMCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tttmc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTTMCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tttmc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTTMCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTMC to value 0"]
impl crate::Resettable for FDCAN_TTTMCrs {
    const RESET_VALUE: u32 = 0;
}
