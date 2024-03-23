#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLRrs>;
#[doc = "Field `SOFC` writer - Start of frame flag clear"]
pub type SOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDDC` writer - Update display done clear"]
pub type UDDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SOFC_W<CLRrs> {
        SOFC_W::new(self, 1)
    }
    #[doc = "Bit 3 - Update display done clear"]
    #[inline(always)]
    #[must_use]
    pub fn uddc(&mut self) -> UDDC_W<CLRrs> {
        UDDC_W::new(self, 3)
    }
}
#[doc = "clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRrs;
impl crate::RegisterSpec for CLRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLRrs {
    const RESET_VALUE: u32 = 0;
}
