#[doc = "Register `LTDC_L1WHPCR` reader"]
pub type R = crate::R<LTDC_L1WHPCRrs>;
#[doc = "Register `LTDC_L1WHPCR` writer"]
pub type W = crate::W<LTDC_L1WHPCRrs>;
#[doc = "Field `WHSTPOS` reader - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\\[11:0\\]
must be ≤ AAW\\[11:0\\]
bits (programmed in LTDC_AWCR register)."]
pub type WHSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSTPOS` writer - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\\[11:0\\]
must be ≤ AAW\\[11:0\\]
bits (programmed in LTDC_AWCR register)."]
pub type WHSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WHSPPOS` reader - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\\[11:0\\]
must be ≥ AHBP\\[11:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
pub type WHSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSPPOS` writer - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\\[11:0\\]
must be ≥ AHBP\\[11:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
pub type WHSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\\[11:0\\]
must be ≤ AAW\\[11:0\\]
bits (programmed in LTDC_AWCR register)."]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\\[11:0\\]
must be ≥ AHBP\\[11:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\\[11:0\\]
must be ≤ AAW\\[11:0\\]
bits (programmed in LTDC_AWCR register)."]
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<LTDC_L1WHPCRrs> {
        WHSTPOS_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\\[11:0\\]
must be ≥ AHBP\\[11:0\\]
bits + 1 (programmed in LTDC_BPCR register)."]
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<LTDC_L1WHPCRrs> {
        WHSPPOS_W::new(self, 16)
    }
}
#[doc = "LTDC layer 1 window horizontal position configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1whpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1whpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1WHPCRrs;
impl crate::RegisterSpec for LTDC_L1WHPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1whpcr::R`](R) reader structure"]
impl crate::Readable for LTDC_L1WHPCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1whpcr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1WHPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1WHPCR to value 0"]
impl crate::Resettable for LTDC_L1WHPCRrs {
    const RESET_VALUE: u32 = 0;
}
