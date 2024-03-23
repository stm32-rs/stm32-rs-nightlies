#[doc = "Register `RCC_APB3DIVR` reader"]
pub type R = crate::R<RCC_APB3DIVRrs>;
#[doc = "Register `RCC_APB3DIVR` writer"]
pub type W = crate::W<RCC_APB3DIVRrs>;
#[doc = "Field `APB3DIV` reader - APB3DIV"]
pub type APB3DIV_R = crate::FieldReader;
#[doc = "Field `APB3DIV` writer - APB3DIV"]
pub type APB3DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB3DIVRDY` reader - APB3DIVRDY"]
pub type APB3DIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&self) -> APB3DIV_R {
        APB3DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - APB3DIVRDY"]
    #[inline(always)]
    pub fn apb3divrdy(&self) -> APB3DIVRDY_R {
        APB3DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    #[must_use]
    pub fn apb3div(&mut self) -> APB3DIV_W<RCC_APB3DIVRrs> {
        APB3DIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB3DIVRrs;
impl crate::RegisterSpec for RCC_APB3DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb3divr::R`](R) reader structure"]
impl crate::Readable for RCC_APB3DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb3divr::W`](W) writer structure"]
impl crate::Writable for RCC_APB3DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB3DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB3DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
