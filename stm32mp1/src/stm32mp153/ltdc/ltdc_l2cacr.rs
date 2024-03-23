#[doc = "Register `LTDC_L2CACR` reader"]
pub type R = crate::R<LTDC_L2CACRrs>;
#[doc = "Register `LTDC_L2CACR` writer"]
pub type W = crate::W<LTDC_L2CACRrs>;
#[doc = "Field `CONSTA` reader - CONSTA"]
pub type CONSTA_R = crate::FieldReader;
#[doc = "Field `CONSTA` writer - CONSTA"]
pub type CONSTA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CONSTA"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CONSTA"]
    #[inline(always)]
    #[must_use]
    pub fn consta(&mut self) -> CONSTA_W<LTDC_L2CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
#[doc = "This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l2cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l2cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L2CACRrs;
impl crate::RegisterSpec for LTDC_L2CACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l2cacr::R`](R) reader structure"]
impl crate::Readable for LTDC_L2CACRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l2cacr::W`](W) writer structure"]
impl crate::Writable for LTDC_L2CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L2CACR to value 0xff"]
impl crate::Resettable for LTDC_L2CACRrs {
    const RESET_VALUE: u32 = 0xff;
}
