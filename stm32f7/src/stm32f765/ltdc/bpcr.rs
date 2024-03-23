#[doc = "Register `BPCR` reader"]
pub type R = crate::R<BPCRrs>;
#[doc = "Register `BPCR` writer"]
pub type W = crate::W<BPCRrs>;
#[doc = "Field `AVBP` reader - Accumulated Vertical back porch (in units of horizontal scan line)"]
pub type AVBP_R = crate::FieldReader<u16>;
#[doc = "Field `AVBP` writer - Accumulated Vertical back porch (in units of horizontal scan line)"]
pub type AVBP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 11, u16>;
#[doc = "Field `AHBP` reader - Accumulated Horizontal back porch (in units of pixel clock period)"]
pub type AHBP_R = crate::FieldReader<u16>;
#[doc = "Field `AHBP` writer - Accumulated Horizontal back porch (in units of pixel clock period)"]
pub type AHBP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline(always)]
    #[must_use]
    pub fn avbp(&mut self) -> AVBP_W<BPCRrs> {
        AVBP_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<BPCRrs> {
        AHBP_W::new(self, 16)
    }
}
#[doc = "Back Porch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPCRrs;
impl crate::RegisterSpec for BPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpcr::R`](R) reader structure"]
impl crate::Readable for BPCRrs {}
#[doc = "`write(|w| ..)` method takes [`bpcr::W`](W) writer structure"]
impl crate::Writable for BPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPCR to value 0"]
impl crate::Resettable for BPCRrs {
    const RESET_VALUE: u32 = 0;
}
