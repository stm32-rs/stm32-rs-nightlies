#[doc = "Register `DSI_VVBPCR` reader"]
pub type R = crate::R<DSI_VVBPCRrs>;
#[doc = "Register `DSI_VVBPCR` writer"]
pub type W = crate::W<DSI_VVBPCRrs>;
#[doc = "Field `VBP` reader - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
pub type VBP_R = crate::FieldReader<u16>;
#[doc = "Field `VBP` writer - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<DSI_VVBPCRrs> {
        VBP_W::new(self, 0)
    }
}
#[doc = "DSI Host video VBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVBPCRrs;
impl crate::RegisterSpec for DSI_VVBPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvbpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VVBPCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvbpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VVBPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_VVBPCR to value 0"]
impl crate::Resettable for DSI_VVBPCRrs {
    const RESET_VALUE: u32 = 0;
}
