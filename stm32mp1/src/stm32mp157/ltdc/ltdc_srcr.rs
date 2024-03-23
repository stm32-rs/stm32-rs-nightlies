#[doc = "Register `LTDC_SRCR` reader"]
pub type R = crate::R<LTDC_SRCRrs>;
#[doc = "Register `LTDC_SRCR` writer"]
pub type W = crate::W<LTDC_SRCRrs>;
#[doc = "Field `IMR` reader - IMR"]
pub type IMR_R = crate::BitReader;
#[doc = "Field `IMR` writer - IMR"]
pub type IMR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBR` reader - VBR"]
pub type VBR_R = crate::BitReader;
#[doc = "Field `VBR` writer - VBR"]
pub type VBR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IMR"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBR"]
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IMR"]
    #[inline(always)]
    #[must_use]
    pub fn imr(&mut self) -> IMR_W<LTDC_SRCRrs> {
        IMR_W::new(self, 0)
    }
    #[doc = "Bit 1 - VBR"]
    #[inline(always)]
    #[must_use]
    pub fn vbr(&mut self) -> VBR_W<LTDC_SRCRrs> {
        VBR_W::new(self, 1)
    }
}
#[doc = "This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_srcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_srcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_SRCRrs;
impl crate::RegisterSpec for LTDC_SRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_srcr::R`](R) reader structure"]
impl crate::Readable for LTDC_SRCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_srcr::W`](W) writer structure"]
impl crate::Writable for LTDC_SRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_SRCR to value 0"]
impl crate::Resettable for LTDC_SRCRrs {
    const RESET_VALUE: u32 = 0;
}
