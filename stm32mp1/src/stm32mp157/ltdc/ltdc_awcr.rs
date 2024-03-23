#[doc = "Register `LTDC_AWCR` reader"]
pub type R = crate::R<LTDC_AWCRrs>;
#[doc = "Register `LTDC_AWCR` writer"]
pub type W = crate::W<LTDC_AWCRrs>;
#[doc = "Field `AAH` reader - AAH"]
pub type AAH_R = crate::FieldReader<u16>;
#[doc = "Field `AAH` writer - AAH"]
pub type AAH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AAW` reader - AAW"]
pub type AAW_R = crate::FieldReader<u16>;
#[doc = "Field `AAW` writer - AAW"]
pub type AAW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - AAH"]
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - AAW"]
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AAH"]
    #[inline(always)]
    #[must_use]
    pub fn aah(&mut self) -> AAH_W<LTDC_AWCRrs> {
        AAH_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - AAW"]
    #[inline(always)]
    #[must_use]
    pub fn aaw(&mut self) -> AAW_W<LTDC_AWCRrs> {
        AAW_W::new(self, 16)
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_awcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_awcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_AWCRrs;
impl crate::RegisterSpec for LTDC_AWCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_awcr::R`](R) reader structure"]
impl crate::Readable for LTDC_AWCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_awcr::W`](W) writer structure"]
impl crate::Writable for LTDC_AWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_AWCR to value 0"]
impl crate::Resettable for LTDC_AWCRrs {
    const RESET_VALUE: u32 = 0;
}
