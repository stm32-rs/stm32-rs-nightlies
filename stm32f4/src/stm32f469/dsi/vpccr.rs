#[doc = "Register `VPCCR` reader"]
pub type R = crate::R<VPCCRrs>;
#[doc = "Register `VPCCR` writer"]
pub type W = crate::W<VPCCRrs>;
#[doc = "Field `VPSIZE` reader - Video Packet Size"]
pub type VPSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `VPSIZE` writer - Video Packet Size"]
pub type VPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Video Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<VPCCRrs> {
        VPSIZE_W::new(self, 0)
    }
}
#[doc = "DSI Host Video Packet Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VPCCRrs;
impl crate::RegisterSpec for VPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vpccr::R`](R) reader structure"]
impl crate::Readable for VPCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vpccr::W`](W) writer structure"]
impl crate::Writable for VPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VPCCR to value 0"]
impl crate::Resettable for VPCCRrs {
    const RESET_VALUE: u32 = 0;
}
