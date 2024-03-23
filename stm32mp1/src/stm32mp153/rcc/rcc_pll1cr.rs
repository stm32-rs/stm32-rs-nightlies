#[doc = "Register `RCC_PLL1CR` reader"]
pub type R = crate::R<RCC_PLL1CRrs>;
#[doc = "Register `RCC_PLL1CR` writer"]
pub type W = crate::W<RCC_PLL1CRrs>;
#[doc = "Field `PLLON` reader - PLLON"]
pub type PLLON_R = crate::BitReader;
#[doc = "Field `PLLON` writer - PLLON"]
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RDY` reader - PLL1RDY"]
pub type PLL1RDY_R = crate::BitReader;
#[doc = "Field `SSCG_CTRL` reader - SSCG_CTRL"]
pub type SSCG_CTRL_R = crate::BitReader;
#[doc = "Field `SSCG_CTRL` writer - SSCG_CTRL"]
pub type SSCG_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVPEN` reader - DIVPEN"]
pub type DIVPEN_R = crate::BitReader;
#[doc = "Field `DIVPEN` writer - DIVPEN"]
pub type DIVPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVQEN` reader - DIVQEN"]
pub type DIVQEN_R = crate::BitReader;
#[doc = "Field `DIVQEN` writer - DIVQEN"]
pub type DIVQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVREN` reader - DIVREN"]
pub type DIVREN_R = crate::BitReader;
#[doc = "Field `DIVREN` writer - DIVREN"]
pub type DIVREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1RDY"]
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    pub fn sscg_ctrl(&self) -> SSCG_CTRL_R {
        SSCG_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    pub fn divpen(&self) -> DIVPEN_R {
        DIVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    pub fn divqen(&self) -> DIVQEN_R {
        DIVQEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    pub fn divren(&self) -> DIVREN_R {
        DIVREN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<RCC_PLL1CRrs> {
        PLLON_W::new(self, 0)
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_ctrl(&mut self) -> SSCG_CTRL_W<RCC_PLL1CRrs> {
        SSCG_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    #[must_use]
    pub fn divpen(&mut self) -> DIVPEN_W<RCC_PLL1CRrs> {
        DIVPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    #[must_use]
    pub fn divqen(&mut self) -> DIVQEN_W<RCC_PLL1CRrs> {
        DIVQEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    #[must_use]
    pub fn divren(&mut self) -> DIVREN_W<RCC_PLL1CRrs> {
        DIVREN_W::new(self, 6)
    }
}
#[doc = "This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PLL1CRrs;
impl crate::RegisterSpec for RCC_PLL1CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pll1cr::R`](R) reader structure"]
impl crate::Readable for RCC_PLL1CRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pll1cr::W`](W) writer structure"]
impl crate::Writable for RCC_PLL1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLL1CR to value 0"]
impl crate::Resettable for RCC_PLL1CRrs {
    const RESET_VALUE: u32 = 0;
}
