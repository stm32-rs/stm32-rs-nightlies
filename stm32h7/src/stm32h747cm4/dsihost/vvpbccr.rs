#[doc = "Register `VVPBCCR` reader"]
pub type R = crate::R<VVPBCCRrs>;
#[doc = "Register `VVPBCCR` writer"]
pub type W = crate::W<VVPBCCRrs>;
#[doc = "Field `VBP` reader - Vertical back"]
pub type VBP_R = crate::FieldReader<u16>;
#[doc = "Field `VBP` writer - Vertical back"]
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical back"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical back"]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<VVPBCCRrs> {
        VBP_W::new(self, 0)
    }
}
#[doc = "DSI Host video VBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvpbccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvpbccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVPBCCRrs;
impl crate::RegisterSpec for VVPBCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvpbccr::R`](R) reader structure"]
impl crate::Readable for VVPBCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vvpbccr::W`](W) writer structure"]
impl crate::Writable for VVPBCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVPBCCR to value 0"]
impl crate::Resettable for VVPBCCRrs {
    const RESET_VALUE: u32 = 0;
}
