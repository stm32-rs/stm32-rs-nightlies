#[doc = "Register `GPIOZ_SECCFGR` writer"]
pub type W = crate::W<GPIOZ_SECCFGRrs>;
#[doc = "Field `SEC0` writer - SEC0"]
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC1` writer - SEC1"]
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC2` writer - SEC2"]
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC3` writer - SEC3"]
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC4` writer - SEC4"]
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC5` writer - SEC5"]
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC6` writer - SEC6"]
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC7` writer - SEC7"]
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SEC0"]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<GPIOZ_SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - SEC1"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<GPIOZ_SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - SEC2"]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<GPIOZ_SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - SEC3"]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<GPIOZ_SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - SEC4"]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<GPIOZ_SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - SEC5"]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<GPIOZ_SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - SEC6"]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<GPIOZ_SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - SEC7"]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<GPIOZ_SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
}
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOZ_SECCFGRrs;
impl crate::RegisterSpec for GPIOZ_SECCFGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioz_seccfgr::W`](W) writer structure"]
impl crate::Writable for GPIOZ_SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOZ_SECCFGR to value 0xff"]
impl crate::Resettable for GPIOZ_SECCFGRrs {
    const RESET_VALUE: u32 = 0xff;
}
