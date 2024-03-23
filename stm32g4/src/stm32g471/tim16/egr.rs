#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGRrs>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCG(1-1)` writer - Capture/compare %s generation"]
pub type CCG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` writer - Break generation"]
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Capture/compare (1-1) generation"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1G` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccg(&mut self, n: u8) -> CCG_W<EGRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCG_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 1)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<EGRrs> {
        COMG_W::new(self, 5)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<EGRrs> {
        BG_W::new(self, 7)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u32 = 0;
}
