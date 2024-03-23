#[doc = "Register `RCC_RTCDIVR` reader"]
pub type R = crate::R<RCC_RTCDIVRrs>;
#[doc = "Register `RCC_RTCDIVR` writer"]
pub type W = crate::W<RCC_RTCDIVRrs>;
#[doc = "Field `RTCDIV` reader - RTCDIV"]
pub type RTCDIV_R = crate::FieldReader;
#[doc = "Field `RTCDIV` writer - RTCDIV"]
pub type RTCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - RTCDIV"]
    #[inline(always)]
    pub fn rtcdiv(&self) -> RTCDIV_R {
        RTCDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RTCDIV"]
    #[inline(always)]
    #[must_use]
    pub fn rtcdiv(&mut self) -> RTCDIV_W<RCC_RTCDIVRrs> {
        RTCDIV_W::new(self, 0)
    }
}
#[doc = "This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rtcdivr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rtcdivr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_RTCDIVRrs;
impl crate::RegisterSpec for RCC_RTCDIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_rtcdivr::R`](R) reader structure"]
impl crate::Readable for RCC_RTCDIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_rtcdivr::W`](W) writer structure"]
impl crate::Writable for RCC_RTCDIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_RTCDIVR to value 0"]
impl crate::Resettable for RCC_RTCDIVRrs {
    const RESET_VALUE: u32 = 0;
}
