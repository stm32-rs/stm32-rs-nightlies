#[doc = "Register `RCC_APB1DIVR` reader"]
pub type R = crate::R<RCC_APB1DIVRrs>;
#[doc = "Register `RCC_APB1DIVR` writer"]
pub type W = crate::W<RCC_APB1DIVRrs>;
#[doc = "Field `APB1DIV` reader - APB1DIV"]
pub type APB1DIV_R = crate::FieldReader;
#[doc = "Field `APB1DIV` writer - APB1DIV"]
pub type APB1DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB1DIVRDY` reader - APB1DIVRDY"]
pub type APB1DIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - APB1DIV"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - APB1DIVRDY"]
    #[inline(always)]
    pub fn apb1divrdy(&self) -> APB1DIVRDY_R {
        APB1DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB1DIV"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> APB1DIV_W<RCC_APB1DIVRrs> {
        APB1DIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB1DIVRrs;
impl crate::RegisterSpec for RCC_APB1DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1divr::R`](R) reader structure"]
impl crate::Readable for RCC_APB1DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1divr::W`](W) writer structure"]
impl crate::Writable for RCC_APB1DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB1DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB1DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
