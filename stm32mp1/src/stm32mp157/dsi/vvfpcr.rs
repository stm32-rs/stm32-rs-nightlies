#[doc = "Register `VVFPCR` reader"]
pub type R = crate::R<VVFPCRrs>;
#[doc = "Register `VVFPCR` writer"]
pub type W = crate::W<VVFPCRrs>;
#[doc = "Field `VFP` reader - VFP"]
pub type VFP_R = crate::FieldReader<u16>;
#[doc = "Field `VFP` writer - VFP"]
pub type VFP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    #[must_use]
    pub fn vfp(&mut self) -> VFP_W<VVFPCRrs> {
        VFP_W::new(self, 0)
    }
}
#[doc = "DSI Host video VFP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvfpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVFPCRrs;
impl crate::RegisterSpec for VVFPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvfpcr::R`](R) reader structure"]
impl crate::Readable for VVFPCRrs {}
#[doc = "`write(|w| ..)` method takes [`vvfpcr::W`](W) writer structure"]
impl crate::Writable for VVFPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVFPCR to value 0"]
impl crate::Resettable for VVFPCRrs {
    const RESET_VALUE: u32 = 0;
}
