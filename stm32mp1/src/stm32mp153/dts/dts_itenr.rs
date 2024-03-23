#[doc = "Register `DTS_ITENR` reader"]
pub type R = crate::R<DTS_ITENRrs>;
#[doc = "Register `DTS_ITENR` writer"]
pub type W = crate::W<DTS_ITENRrs>;
#[doc = "Field `TS1_ITEEN` reader - TS1_ITEEN"]
pub type TS1_ITEEN_R = crate::BitReader;
#[doc = "Field `TS1_ITEEN` writer - TS1_ITEEN"]
pub type TS1_ITEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_ITLEN` reader - TS1_ITLEN"]
pub type TS1_ITLEN_R = crate::BitReader;
#[doc = "Field `TS1_ITLEN` writer - TS1_ITLEN"]
pub type TS1_ITLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_ITHEN` reader - TS1_ITHEN"]
pub type TS1_ITHEN_R = crate::BitReader;
#[doc = "Field `TS1_ITHEN` writer - TS1_ITHEN"]
pub type TS1_ITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_AITEEN` reader - TS1_AITEEN"]
pub type TS1_AITEEN_R = crate::BitReader;
#[doc = "Field `TS1_AITEEN` writer - TS1_AITEEN"]
pub type TS1_AITEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_AITLEN` reader - TS1_AITLEN"]
pub type TS1_AITLEN_R = crate::BitReader;
#[doc = "Field `TS1_AITLEN` writer - TS1_AITLEN"]
pub type TS1_AITLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_AITHEN` reader - TS1_AITHEN"]
pub type TS1_AITHEN_R = crate::BitReader;
#[doc = "Field `TS1_AITHEN` writer - TS1_AITHEN"]
pub type TS1_AITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TS1_ITEEN"]
    #[inline(always)]
    pub fn ts1_iteen(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TS1_ITLEN"]
    #[inline(always)]
    pub fn ts1_itlen(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TS1_ITHEN"]
    #[inline(always)]
    pub fn ts1_ithen(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TS1_AITEEN"]
    #[inline(always)]
    pub fn ts1_aiteen(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TS1_AITLEN"]
    #[inline(always)]
    pub fn ts1_aitlen(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TS1_AITHEN"]
    #[inline(always)]
    pub fn ts1_aithen(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_ITEEN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_iteen(&mut self) -> TS1_ITEEN_W<DTS_ITENRrs> {
        TS1_ITEEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TS1_ITLEN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_itlen(&mut self) -> TS1_ITLEN_W<DTS_ITENRrs> {
        TS1_ITLEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TS1_ITHEN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_ithen(&mut self) -> TS1_ITHEN_W<DTS_ITENRrs> {
        TS1_ITHEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - TS1_AITEEN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_aiteen(&mut self) -> TS1_AITEEN_W<DTS_ITENRrs> {
        TS1_AITEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TS1_AITLEN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_aitlen(&mut self) -> TS1_AITLEN_W<DTS_ITENRrs> {
        TS1_AITLEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TS1_AITHEN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_aithen(&mut self) -> TS1_AITHEN_W<DTS_ITENRrs> {
        TS1_AITHEN_W::new(self, 6)
    }
}
#[doc = "Temperature sensor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_itenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_itenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_ITENRrs;
impl crate::RegisterSpec for DTS_ITENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts_itenr::R`](R) reader structure"]
impl crate::Readable for DTS_ITENRrs {}
#[doc = "`write(|w| ..)` method takes [`dts_itenr::W`](W) writer structure"]
impl crate::Writable for DTS_ITENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTS_ITENR to value 0"]
impl crate::Resettable for DTS_ITENRrs {
    const RESET_VALUE: u32 = 0;
}
