#[doc = "Register `LTDC_L1WVPCR` reader"]
pub type R = crate::R<LTDC_L1WVPCRrs>;
#[doc = "Register `LTDC_L1WVPCR` writer"]
pub type W = crate::W<LTDC_L1WVPCRrs>;
#[doc = "Field `WVSTPOS` reader - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\\[10:0\\]
must be ≤ AAH\\[10:0\\]
bits (programmed in LTDC_AWCR register)."]
pub type WVSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WVSTPOS` writer - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\\[10:0\\]
must be ≤ AAH\\[10:0\\]
bits (programmed in LTDC_AWCR register)."]
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `WVSPPOS` reader - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\\[10:0\\]
must be ≥ AVBP\\[10:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
pub type WVSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WVSPPOS` writer - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\\[10:0\\]
must be ≥ AVBP\\[10:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\\[10:0\\]
must be ≤ AAH\\[10:0\\]
bits (programmed in LTDC_AWCR register)."]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\\[10:0\\]
must be ≥ AVBP\\[10:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\\[10:0\\]
must be ≤ AAH\\[10:0\\]
bits (programmed in LTDC_AWCR register)."]
    #[inline(always)]
    #[must_use]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<LTDC_L1WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\\[10:0\\]
must be ≥ AVBP\\[10:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
    #[inline(always)]
    #[must_use]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<LTDC_L1WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
#[doc = "LTDC layer 1 window vertical position configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1wvpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1wvpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
