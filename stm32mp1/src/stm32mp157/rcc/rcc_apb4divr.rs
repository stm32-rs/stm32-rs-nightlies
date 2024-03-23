#[doc = "Register `RCC_APB4DIVR` reader"]
pub type R = crate::R<RCC_APB4DIVRrs>;
#[doc = "Register `RCC_APB4DIVR` writer"]
pub type W = crate::W<RCC_APB4DIVRrs>;
#[doc = "Field `APB4DIV` reader - APB4DIV"]
pub type APB4DIV_R = crate::FieldReader;
#[doc = "Field `APB4DIV` writer - APB4DIV"]
pub type APB4DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB4DIVRDY` reader - APB4DIVRDY"]
pub type APB4DIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - APB4DIV"]
    #[inline(always)]
    pub fn apb4div(&self) -> APB4DIV_R {
        APB4DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - APB4DIVRDY"]
    #[inline(always)]
    pub fn apb4divrdy(&self) -> APB4DIVRDY_R {
        APB4DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB4DIV"]
    #[inline(always)]
    #[must_use]
    pub fn apb4div(&mut self) -> APB4DIV_W<RCC_APB4DIVRrs> {
        APB4DIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb4divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb4divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB4DIVRrs;
impl crate::RegisterSpec for RCC_APB4DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb4divr::R`](R) reader structure"]
impl crate::Readable for RCC_APB4DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb4divr::W`](W) writer structure"]
impl crate::Writable for RCC_APB4DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB4DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB4DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
