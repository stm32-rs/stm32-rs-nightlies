#[doc = "Register `DSI_DPDL1SRCR` reader"]
pub type R = crate::R<DSI_DPDL1SRCRrs>;
#[doc = "Register `DSI_DPDL1SRCR` writer"]
pub type W = crate::W<DSI_DPDL1SRCRrs>;
#[doc = "Field `SRC` reader - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved"]
pub type SRC_R = crate::FieldReader;
#[doc = "Field `SRC` writer - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved"]
pub type SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<DSI_DPDL1SRCRrs> {
        SRC_W::new(self, 0)
    }
}
#[doc = "DSI D-PHY data lane 1 skew rate control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpdl1srcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpdl1srcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_DPDL1SRCRrs;
impl crate::RegisterSpec for DSI_DPDL1SRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dpdl1srcr::R`](R) reader structure"]
impl crate::Readable for DSI_DPDL1SRCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_dpdl1srcr::W`](W) writer structure"]
impl crate::Writable for DSI_DPDL1SRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_DPDL1SRCR to value 0"]
impl crate::Resettable for DSI_DPDL1SRCRrs {
    const RESET_VALUE: u32 = 0;
}
