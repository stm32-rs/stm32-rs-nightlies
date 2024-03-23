#[doc = "Register `LTDC_L1WVPCR` reader"]
pub type R = crate::R<LTDC_L1WVPCRrs>;
#[doc = "Register `LTDC_L1WVPCR` writer"]
pub type W = crate::W<LTDC_L1WVPCRrs>;
#[doc = "Field `WVSTPOS` reader - WVSTPOS"]
pub type WVSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WVSTPOS` writer - WVSTPOS"]
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WVSPPOS` reader - WVSPPOS"]
pub type WVSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WVSPPOS` writer - WVSPPOS"]
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<LTDC_L1WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    #[must_use]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<LTDC_L1WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1wvpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1wvpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1WVPCRrs;
impl crate::RegisterSpec for LTDC_L1WVPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1wvpcr::R`](R) reader structure"]
impl crate::Readable for LTDC_L1WVPCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1wvpcr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1WVPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1WVPCR to value 0"]
impl crate::Resettable for LTDC_L1WVPCRrs {
    const RESET_VALUE: u32 = 0;
}
