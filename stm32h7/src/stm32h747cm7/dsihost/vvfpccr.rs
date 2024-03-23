#[doc = "Register `VVFPCCR` reader"]
pub type R = crate::R<VVFPCCRrs>;
#[doc = "Register `VVFPCCR` writer"]
pub type W = crate::W<VVFPCCRrs>;
#[doc = "Field `VFP` reader - Vertical front"]
pub type VFP_R = crate::FieldReader<u16>;
#[doc = "Field `VFP` writer - Vertical front"]
pub type VFP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical front"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical front"]
    #[inline(always)]
    #[must_use]
    pub fn vfp(&mut self) -> VFP_W<VVFPCCRrs> {
        VFP_W::new(self, 0)
    }
}
#[doc = "DSI Host video VFP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvfpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVFPCCRrs;
impl crate::RegisterSpec for VVFPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvfpccr::R`](R) reader structure"]
impl crate::Readable for VVFPCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vvfpccr::W`](W) writer structure"]
impl crate::Writable for VVFPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVFPCCR to value 0"]
impl crate::Resettable for VVFPCCRrs {
    const RESET_VALUE: u32 = 0;
}
