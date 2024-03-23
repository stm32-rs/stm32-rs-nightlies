#[doc = "Register `LTDC_TWCR` reader"]
pub type R = crate::R<LTDC_TWCRrs>;
#[doc = "Register `LTDC_TWCR` writer"]
pub type W = crate::W<LTDC_TWCRrs>;
#[doc = "Field `TOTALH` reader - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1."]
pub type TOTALH_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALH` writer - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1."]
pub type TOTALH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `TOTALW` reader - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1."]
pub type TOTALW_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALW` writer - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1."]
pub type TOTALW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1."]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1."]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn totalh(&mut self) -> TOTALH_W<LTDC_TWCRrs> {
        TOTALH_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn totalw(&mut self) -> TOTALW_W<LTDC_TWCRrs> {
        TOTALW_W::new(self, 16)
    }
}
#[doc = "LTDC total width configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_twcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_twcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_TWCRrs;
impl crate::RegisterSpec for LTDC_TWCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_twcr::R`](R) reader structure"]
impl crate::Readable for LTDC_TWCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_twcr::W`](W) writer structure"]
impl crate::Writable for LTDC_TWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_TWCR to value 0"]
impl crate::Resettable for LTDC_TWCRrs {
    const RESET_VALUE: u32 = 0;
}
