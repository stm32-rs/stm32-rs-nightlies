#[doc = "Register `DMAMUX_RGCFR` writer"]
pub type W = crate::W<DMAMUX_RGCFRrs>;
#[doc = "Field `COF0` writer - COF0"]
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF1` writer - COF1"]
pub type COF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF2` writer - COF2"]
pub type COF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF3` writer - COF3"]
pub type COF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF4` writer - COF4"]
pub type COF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF5` writer - COF5"]
pub type COF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF6` writer - COF6"]
pub type COF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF7` writer - COF7"]
pub type COF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - COF0"]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> COF0_W<DMAMUX_RGCFRrs> {
        COF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - COF1"]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> COF1_W<DMAMUX_RGCFRrs> {
        COF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - COF2"]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> COF2_W<DMAMUX_RGCFRrs> {
        COF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - COF3"]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> COF3_W<DMAMUX_RGCFRrs> {
        COF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - COF4"]
    #[inline(always)]
    #[must_use]
    pub fn cof4(&mut self) -> COF4_W<DMAMUX_RGCFRrs> {
        COF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - COF5"]
    #[inline(always)]
    #[must_use]
    pub fn cof5(&mut self) -> COF5_W<DMAMUX_RGCFRrs> {
        COF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - COF6"]
    #[inline(always)]
    #[must_use]
    pub fn cof6(&mut self) -> COF6_W<DMAMUX_RGCFRrs> {
        COF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - COF7"]
    #[inline(always)]
    #[must_use]
    pub fn cof7(&mut self) -> COF7_W<DMAMUX_RGCFRrs> {
        COF7_W::new(self, 7)
    }
}
#[doc = "DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_RGCFRrs;
impl crate::RegisterSpec for DMAMUX_RGCFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmamux_rgcfr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_RGCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_RGCFR to value 0"]
impl crate::Resettable for DMAMUX_RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
