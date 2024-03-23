#[doc = "Register `VCCCR` reader"]
pub type R = crate::R<VCCCRrs>;
#[doc = "Register `VCCCR` writer"]
pub type W = crate::W<VCCCRrs>;
#[doc = "Field `NUMC` reader - Number of chunks"]
pub type NUMC_R = crate::FieldReader<u16>;
#[doc = "Field `NUMC` writer - Number of chunks"]
pub type NUMC_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Number of chunks"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of chunks"]
    #[inline(always)]
    #[must_use]
    pub fn numc(&mut self) -> NUMC_W<VCCCRrs> {
        NUMC_W::new(self, 0)
    }
}
#[doc = "DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vcccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vcccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VCCCRrs;
impl crate::RegisterSpec for VCCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vcccr::R`](R) reader structure"]
impl crate::Readable for VCCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vcccr::W`](W) writer structure"]
impl crate::Writable for VCCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VCCCR to value 0"]
impl crate::Resettable for VCCCRrs {
    const RESET_VALUE: u32 = 0;
}
