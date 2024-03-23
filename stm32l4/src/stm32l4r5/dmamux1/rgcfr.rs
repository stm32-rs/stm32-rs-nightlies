#[doc = "Register `RGCFR` writer"]
pub type W = crate::W<RGCFRrs>;
#[doc = "Field `COF0` writer - Clear trigger overrun event flag"]
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF1` writer - Clear trigger overrun event flag"]
pub type COF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF2` writer - Clear trigger overrun event flag"]
pub type COF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF3` writer - Clear trigger overrun event flag"]
pub type COF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear trigger overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> COF0_W<RGCFRrs> {
        COF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> COF1_W<RGCFRrs> {
        COF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> COF2_W<RGCFRrs> {
        COF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear trigger overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> COF3_W<RGCFRrs> {
        COF3_W::new(self, 3)
    }
}
#[doc = "request generator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure"]
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
