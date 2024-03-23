#[doc = "Register `LTDC_L2WHPCR` reader"]
pub type R = crate::R<LTDC_L2WHPCRrs>;
#[doc = "Register `LTDC_L2WHPCR` writer"]
pub type W = crate::W<LTDC_L2WHPCRrs>;
#[doc = "Field `WHSTPOS` reader - WHSTPOS"]
pub type WHSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSTPOS` writer - WHSTPOS"]
pub type WHSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WHSPPOS` reader - WHSPPOS"]
pub type WHSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSPPOS` writer - WHSPPOS"]
pub type WHSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - WHSTPOS"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WHSPPOS"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WHSTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<LTDC_L2WHPCRrs> {
        WHSTPOS_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - WHSPPOS"]
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<LTDC_L2WHPCRrs> {
        WHSPPOS_W::new(self, 16)
    }
}
#[doc = "This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l2whpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l2whpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L2WHPCRrs;
impl crate::RegisterSpec for LTDC_L2WHPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l2whpcr::R`](R) reader structure"]
impl crate::Readable for LTDC_L2WHPCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l2whpcr::W`](W) writer structure"]
impl crate::Writable for LTDC_L2WHPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L2WHPCR to value 0"]
impl crate::Resettable for LTDC_L2WHPCRrs {
    const RESET_VALUE: u32 = 0;
}
