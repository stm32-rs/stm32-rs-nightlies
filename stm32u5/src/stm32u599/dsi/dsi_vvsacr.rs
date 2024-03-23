#[doc = "Register `DSI_VVSACR` reader"]
pub type R = crate::R<DSI_VVSACRrs>;
#[doc = "Register `DSI_VVSACR` writer"]
pub type W = crate::W<DSI_VVSACRrs>;
#[doc = "Field `VSA` reader - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
pub type VSA_R = crate::FieldReader<u16>;
#[doc = "Field `VSA` writer - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
pub type VSA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VSA_W<DSI_VVSACRrs> {
        VSA_W::new(self, 0)
    }
}
#[doc = "DSI Host video VSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvsacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVSACRrs;
impl crate::RegisterSpec for DSI_VVSACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvsacr::R`](R) reader structure"]
impl crate::Readable for DSI_VVSACRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvsacr::W`](W) writer structure"]
impl crate::Writable for DSI_VVSACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_VVSACR to value 0"]
impl crate::Resettable for DSI_VVSACRrs {
    const RESET_VALUE: u32 = 0;
}
