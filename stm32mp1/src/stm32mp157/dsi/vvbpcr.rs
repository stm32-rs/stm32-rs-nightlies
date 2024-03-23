#[doc = "Register `VVBPCR` reader"]
pub type R = crate::R<VVBPCRrs>;
#[doc = "Register `VVBPCR` writer"]
pub type W = crate::W<VVBPCRrs>;
#[doc = "Field `VBP` reader - VBP"]
pub type VBP_R = crate::FieldReader<u16>;
#[doc = "Field `VBP` writer - VBP"]
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<VVBPCRrs> {
        VBP_W::new(self, 0)
    }
}
#[doc = "DSI Host video VBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVBPCRrs;
impl crate::RegisterSpec for VVBPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvbpcr::R`](R) reader structure"]
impl crate::Readable for VVBPCRrs {}
#[doc = "`write(|w| ..)` method takes [`vvbpcr::W`](W) writer structure"]
impl crate::Writable for VVBPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVBPCR to value 0"]
impl crate::Resettable for VVBPCRrs {
    const RESET_VALUE: u32 = 0;
}
