#[doc = "Register `RCC_RCK4SELR` reader"]
pub type R = crate::R<RCC_RCK4SELRrs>;
#[doc = "Register `RCC_RCK4SELR` writer"]
pub type W = crate::W<RCC_RCK4SELRrs>;
#[doc = "Field `PLL4SRC` reader - PLL4SRC"]
pub type PLL4SRC_R = crate::FieldReader;
#[doc = "Field `PLL4SRC` writer - PLL4SRC"]
pub type PLL4SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL4SRCRDY` reader - PLL4SRCRDY"]
pub type PLL4SRCRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - PLL4SRC"]
    #[inline(always)]
    pub fn pll4src(&self) -> PLL4SRC_R {
        PLL4SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - PLL4SRCRDY"]
    #[inline(always)]
    pub fn pll4srcrdy(&self) -> PLL4SRCRDY_R {
        PLL4SRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL4SRC"]
    #[inline(always)]
    #[must_use]
    pub fn pll4src(&mut self) -> PLL4SRC_W<RCC_RCK4SELRrs> {
        PLL4SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to select the reference clock for PLL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rck4selr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rck4selr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_RCK4SELRrs;
impl crate::RegisterSpec for RCC_RCK4SELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_rck4selr::R`](R) reader structure"]
impl crate::Readable for RCC_RCK4SELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_rck4selr::W`](W) writer structure"]
impl crate::Writable for RCC_RCK4SELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_RCK4SELR to value 0x8000_0000"]
impl crate::Resettable for RCC_RCK4SELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
