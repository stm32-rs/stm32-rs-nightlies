#[doc = "Register `RCC_CIER` reader"]
pub type R = crate::R<RCC_CIERrs>;
#[doc = "Register `RCC_CIER` writer"]
pub type W = crate::W<RCC_CIERrs>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LSERDYIE_R = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSISRDYIE` reader - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
pub type MSISRDYIE_R = crate::BitReader;
#[doc = "Field `MSISRDYIE` writer - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
pub type MSISRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
pub type HSIRDYIE_R = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HSERDYIE_R = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
pub type HSI48RDYIE_R = crate::BitReader;
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RDYIE` reader - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
pub type PLL1RDYIE_R = crate::BitReader;
#[doc = "Field `PLL1RDYIE` writer - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
pub type PLL1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2RDYIE` reader - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock."]
pub type PLL2RDYIE_R = crate::BitReader;
#[doc = "Field `PLL2RDYIE` writer - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock."]
pub type PLL2RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3RDYIE` reader - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock."]
pub type PLL3RDYIE_R = crate::BitReader;
#[doc = "Field `PLL3RDYIE` writer - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock."]
pub type PLL3RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIKRDYIE` reader - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
pub type MSIKRDYIE_R = crate::BitReader;
#[doc = "Field `MSIKRDYIE` writer - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
pub type MSIKRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHSIRDYIE` reader - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
pub type SHSIRDYIE_R = crate::BitReader;
#[doc = "Field `SHSIRDYIE` writer - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
pub type SHSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
    #[inline(always)]
    pub fn msisrdyie(&self) -> MSISRDYIE_R {
        MSISRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock."]
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock."]
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
    #[inline(always)]
    pub fn msikrdyie(&self) -> MSIKRDYIE_R {
        MSIKRDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
    #[inline(always)]
    pub fn shsirdyie(&self) -> SHSIRDYIE_R {
        SHSIRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<RCC_CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<RCC_CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn msisrdyie(&mut self) -> MSISRDYIE_W<RCC_CIERrs> {
        MSISRDYIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<RCC_CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<RCC_CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<RCC_CIERrs> {
        HSI48RDYIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<RCC_CIERrs> {
        PLL1RDYIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock."]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<RCC_CIERrs> {
        PLL2RDYIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock."]
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<RCC_CIERrs> {
        PLL3RDYIE_W::new(self, 8)
    }
    #[doc = "Bit 11 - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn msikrdyie(&mut self) -> MSIKRDYIE_W<RCC_CIERrs> {
        MSIKRDYIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn shsirdyie(&mut self) -> SHSIRDYIE_W<RCC_CIERrs> {
        SHSIRDYIE_W::new(self, 12)
    }
}
#[doc = "RCC clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_CIERrs;
impl crate::RegisterSpec for RCC_CIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cier::R`](R) reader structure"]
impl crate::Readable for RCC_CIERrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_cier::W`](W) writer structure"]
impl crate::Writable for RCC_CIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CIER to value 0"]
impl crate::Resettable for RCC_CIERrs {
    const RESET_VALUE: u32 = 0;
}
