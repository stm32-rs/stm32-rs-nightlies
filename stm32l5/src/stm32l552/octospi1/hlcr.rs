#[doc = "Register `HLCR` reader"]
pub type R = crate::R<HLCRrs>;
#[doc = "Register `HLCR` writer"]
pub type W = crate::W<HLCRrs>;
#[doc = "Field `ALTERNATE` reader - Alternate bytes"]
pub type ALTERNATE_R = crate::FieldReader<u32>;
#[doc = "Field `ALTERNATE` writer - Alternate bytes"]
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alternate bytes"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alternate bytes"]
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<HLCRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
#[doc = "HyperBusTM latency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hlcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hlcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HLCRrs;
impl crate::RegisterSpec for HLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hlcr::R`](R) reader structure"]
impl crate::Readable for HLCRrs {}
#[doc = "`write(|w| ..)` method takes [`hlcr::W`](W) writer structure"]
impl crate::Writable for HLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HLCR to value 0"]
impl crate::Resettable for HLCRrs {
    const RESET_VALUE: u32 = 0;
}
