#[doc = "Register `RCC_APB5DIVR` reader"]
pub type R = crate::R<RCC_APB5DIVRrs>;
#[doc = "Register `RCC_APB5DIVR` writer"]
pub type W = crate::W<RCC_APB5DIVRrs>;
#[doc = "Field `APB5DIV` reader - APB5DIV"]
pub type APB5DIV_R = crate::FieldReader;
#[doc = "Field `APB5DIV` writer - APB5DIV"]
pub type APB5DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB5DIVRDY` reader - APB5DIVRDY"]
pub type APB5DIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - APB5DIV"]
    #[inline(always)]
    pub fn apb5div(&self) -> APB5DIV_R {
        APB5DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - APB5DIVRDY"]
    #[inline(always)]
    pub fn apb5divrdy(&self) -> APB5DIVRDY_R {
        APB5DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB5DIV"]
    #[inline(always)]
    #[must_use]
    pub fn apb5div(&mut self) -> APB5DIV_W<RCC_APB5DIVRrs> {
        APB5DIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb5divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb5divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB5DIVRrs;
impl crate::RegisterSpec for RCC_APB5DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb5divr::R`](R) reader structure"]
impl crate::Readable for RCC_APB5DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb5divr::W`](W) writer structure"]
impl crate::Writable for RCC_APB5DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB5DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB5DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
