#[doc = "Register `RCC_APB2DIVR` reader"]
pub type R = crate::R<RCC_APB2DIVRrs>;
#[doc = "Register `RCC_APB2DIVR` writer"]
pub type W = crate::W<RCC_APB2DIVRrs>;
#[doc = "Field `APB2DIV` reader - APB2DIV"]
pub type APB2DIV_R = crate::FieldReader;
#[doc = "Field `APB2DIV` writer - APB2DIV"]
pub type APB2DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB2DIVRDY` reader - APB2DIVRDY"]
pub type APB2DIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - APB2DIVRDY"]
    #[inline(always)]
    pub fn apb2divrdy(&self) -> APB2DIVRDY_R {
        APB2DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> APB2DIV_W<RCC_APB2DIVRrs> {
        APB2DIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB2DIVRrs;
impl crate::RegisterSpec for RCC_APB2DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2divr::R`](R) reader structure"]
impl crate::Readable for RCC_APB2DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2divr::W`](W) writer structure"]
impl crate::Writable for RCC_APB2DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB2DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB2DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
