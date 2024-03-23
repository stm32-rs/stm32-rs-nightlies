#[doc = "Register `VPCR` reader"]
pub type R = crate::R<VPCRrs>;
#[doc = "Register `VPCR` writer"]
pub type W = crate::W<VPCRrs>;
#[doc = "Field `VPSIZE` reader - Video Packet Size"]
pub type VPSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `VPSIZE` writer - Video Packet Size"]
pub type VPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Video Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<VPCRrs> {
        VPSIZE_W::new(self, 0)
    }
}
#[doc = "DSI Host Video Packet Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VPCRrs;
impl crate::RegisterSpec for VPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vpcr::R`](R) reader structure"]
impl crate::Readable for VPCRrs {}
#[doc = "`write(|w| ..)` method takes [`vpcr::W`](W) writer structure"]
impl crate::Writable for VPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VPCR to value 0"]
impl crate::Resettable for VPCRrs {
    const RESET_VALUE: u32 = 0;
}
