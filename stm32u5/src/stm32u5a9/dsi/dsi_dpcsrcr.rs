#[doc = "Register `DSI_DPCSRCR` reader"]
pub type R = crate::R<DSI_DPCSRCRrs>;
#[doc = "Register `DSI_DPCSRCR` writer"]
pub type W = crate::W<DSI_DPCSRCRrs>;
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
    pub fn src(&mut self) -> SRC_W<DSI_DPCSRCRrs> {
        SRC_W::new(self, 0)
    }
}
#[doc = "DSI D-PHY clock skew rate control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpcsrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpcsrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_DPCSRCRrs;
impl crate::RegisterSpec for DSI_DPCSRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dpcsrcr::R`](R) reader structure"]
impl crate::Readable for DSI_DPCSRCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_dpcsrcr::W`](W) writer structure"]
impl crate::Writable for DSI_DPCSRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_DPCSRCR to value 0"]
impl crate::Resettable for DSI_DPCSRCRrs {
    const RESET_VALUE: u32 = 0;
}
