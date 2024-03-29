#[doc = "Register `TZSC_MPCWM3AR` reader"]
pub type R = crate::R<TZSC_MPCWM3ARrs>;
#[doc = "Register `TZSC_MPCWM3AR` writer"]
pub type W = crate::W<TZSC_MPCWM3ARrs>;
#[doc = "Field `SUBA_START` reader - Start of sub-region A"]
pub type SUBA_START_R = crate::FieldReader<u16>;
#[doc = "Field `SUBA_START` writer - Start of sub-region A"]
pub type SUBA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SUBA_LENGTH` reader - Length of sub-region A"]
pub type SUBA_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `SUBA_LENGTH` writer - Length of sub-region A"]
pub type SUBA_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Start of sub-region A"]
    #[inline(always)]
    pub fn suba_start(&self) -> SUBA_START_R {
        SUBA_START_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Length of sub-region A"]
    #[inline(always)]
    pub fn suba_length(&self) -> SUBA_LENGTH_R {
        SUBA_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Start of sub-region A"]
    #[inline(always)]
    #[must_use]
    pub fn suba_start(&mut self) -> SUBA_START_W<TZSC_MPCWM3ARrs> {
        SUBA_START_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Length of sub-region A"]
    #[inline(always)]
    #[must_use]
    pub fn suba_length(&mut self) -> SUBA_LENGTH_W<TZSC_MPCWM3ARrs> {
        SUBA_LENGTH_W::new(self, 16)
    }
}
#[doc = "TZSC memory 3 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm3ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm3ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_MPCWM3ARrs;
impl crate::RegisterSpec for TZSC_MPCWM3ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_mpcwm3ar::R`](R) reader structure"]
impl crate::Readable for TZSC_MPCWM3ARrs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_mpcwm3ar::W`](W) writer structure"]
impl crate::Writable for TZSC_MPCWM3ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_MPCWM3AR to value 0"]
impl crate::Resettable for TZSC_MPCWM3ARrs {
    const RESET_VALUE: u32 = 0;
}
