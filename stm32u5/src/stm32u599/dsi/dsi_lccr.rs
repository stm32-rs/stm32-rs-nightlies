#[doc = "Register `DSI_LCCR` reader"]
pub type R = crate::R<DSI_LCCRrs>;
#[doc = "Register `DSI_LCCR` writer"]
pub type W = crate::W<DSI_LCCRrs>;
#[doc = "Field `CMDSIZE` reader - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
pub type CMDSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `CMDSIZE` writer - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
pub type CMDSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cmdsize(&mut self) -> CMDSIZE_W<DSI_LCCRrs> {
        CMDSIZE_W::new(self, 0)
    }
}
#[doc = "DSI Host LTDC command configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LCCRrs;
impl crate::RegisterSpec for DSI_LCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lccr::R`](R) reader structure"]
impl crate::Readable for DSI_LCCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_lccr::W`](W) writer structure"]
impl crate::Writable for DSI_LCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_LCCR to value 0"]
impl crate::Resettable for DSI_LCCRrs {
    const RESET_VALUE: u32 = 0;
}
