#[doc = "Register `DSI_VPCR` reader"]
pub type R = crate::R<DSI_VPCRrs>;
#[doc = "Register `DSI_VPCR` writer"]
pub type W = crate::W<DSI_VPCRrs>;
#[doc = "Field `VPSIZE` reader - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
pub type VPSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `VPSIZE` writer - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
pub type VPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<DSI_VPCRrs> {
        VPSIZE_W::new(self, 0)
    }
}
#[doc = "DSI Host video packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VPCRrs;
impl crate::RegisterSpec for DSI_VPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VPCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_vpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_VPCR to value 0"]
impl crate::Resettable for DSI_VPCRrs {
    const RESET_VALUE: u32 = 0;
}
