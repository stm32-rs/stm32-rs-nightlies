#[doc = "Register `VCCR` reader"]
pub type R = crate::R<VCCRrs>;
#[doc = "Register `VCCR` writer"]
pub type W = crate::W<VCCRrs>;
#[doc = "Field `NUMC` reader - Number of Chunks"]
pub type NUMC_R = crate::FieldReader<u16>;
#[doc = "Field `NUMC` writer - Number of Chunks"]
pub type NUMC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Number of Chunks"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Number of Chunks"]
    #[inline(always)]
    #[must_use]
    pub fn numc(&mut self) -> NUMC_W<VCCRrs> {
        NUMC_W::new(self, 0)
    }
}
#[doc = "DSI Host Video Chunks Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VCCRrs;
impl crate::RegisterSpec for VCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vccr::R`](R) reader structure"]
impl crate::Readable for VCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vccr::W`](W) writer structure"]
impl crate::Writable for VCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VCCR to value 0"]
impl crate::Resettable for VCCRrs {
    const RESET_VALUE: u32 = 0;
}
