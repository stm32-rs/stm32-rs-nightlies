#[doc = "Register `AWCR` reader"]
pub type R = crate::R<AWCRrs>;
#[doc = "Register `AWCR` writer"]
pub type W = crate::W<AWCRrs>;
#[doc = "Field `AAH` reader - Accumulated Active Height (in units of horizontal scan line)"]
pub type AAH_R = crate::FieldReader<u16>;
#[doc = "Field `AAH` writer - Accumulated Active Height (in units of horizontal scan line)"]
pub type AAH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AAW` reader - Accumulated Active Width (in units of pixel clock period)"]
pub type AAW_R = crate::FieldReader<u16>;
#[doc = "Field `AAW` writer - Accumulated Active Width (in units of pixel clock period)"]
pub type AAW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Accumulated Active Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    #[must_use]
    pub fn aah(&mut self) -> AAH_W<AWCRrs> {
        AAH_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Accumulated Active Width (in units of pixel clock period)"]
    #[inline(always)]
    #[must_use]
    pub fn aaw(&mut self) -> AAW_W<AWCRrs> {
        AAW_W::new(self, 16)
    }
}
#[doc = "LTDC Active Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWCRrs;
impl crate::RegisterSpec for AWCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awcr::R`](R) reader structure"]
impl crate::Readable for AWCRrs {}
#[doc = "`write(|w| ..)` method takes [`awcr::W`](W) writer structure"]
impl crate::Writable for AWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWCR to value 0"]
impl crate::Resettable for AWCRrs {
    const RESET_VALUE: u32 = 0;
}
