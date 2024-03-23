#[doc = "Register `TDCR` reader"]
pub type R = crate::R<TDCRrs>;
#[doc = "Register `TDCR` writer"]
pub type W = crate::W<TDCRrs>;
#[doc = "Field `TDCF` reader - TDCF"]
pub type TDCF_R = crate::FieldReader;
#[doc = "Field `TDCF` writer - TDCF"]
pub type TDCF_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDCO` reader - TDCO"]
pub type TDCO_R = crate::FieldReader;
#[doc = "Field `TDCO` writer - TDCO"]
pub type TDCO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - TDCF"]
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - TDCO"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TDCF"]
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TDCF_W<TDCRrs> {
        TDCF_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - TDCO"]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TDCO_W<TDCRrs> {
        TDCO_W::new(self, 8)
    }
}
#[doc = "FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDCRrs;
impl crate::RegisterSpec for TDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdcr::R`](R) reader structure"]
impl crate::Readable for TDCRrs {}
#[doc = "`write(|w| ..)` method takes [`tdcr::W`](W) writer structure"]
impl crate::Writable for TDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDCR to value 0"]
impl crate::Resettable for TDCRrs {
    const RESET_VALUE: u32 = 0;
}
