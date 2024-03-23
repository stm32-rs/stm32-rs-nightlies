#[doc = "Register `DSI_VVACR` reader"]
pub type R = crate::R<DSI_VVACRrs>;
#[doc = "Register `DSI_VVACR` writer"]
pub type W = crate::W<DSI_VVACRrs>;
#[doc = "Field `VA` reader - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
pub type VA_R = crate::FieldReader<u16>;
#[doc = "Field `VA` writer - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<DSI_VVACRrs> {
        VA_W::new(self, 0)
    }
}
#[doc = "DSI Host video VA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVACRrs;
impl crate::RegisterSpec for DSI_VVACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvacr::R`](R) reader structure"]
impl crate::Readable for DSI_VVACRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvacr::W`](W) writer structure"]
impl crate::Writable for DSI_VVACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_VVACR to value 0"]
impl crate::Resettable for DSI_VVACRrs {
    const RESET_VALUE: u32 = 0;
}
