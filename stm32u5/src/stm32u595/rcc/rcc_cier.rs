///Register `RCC_CIER` reader
pub type R = crate::R<RCC_CIERrs>;
///Register `RCC_CIER` writer
pub type W = crate::W<RCC_CIERrs>;
///Field `LSIRDYIE` reader - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_R = crate::BitReader;
///Field `LSIRDYIE` writer - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISRDYIE` reader - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
pub type MSISRDYIE_R = crate::BitReader;
///Field `MSISRDYIE` writer - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
pub type MSISRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub type HSI48RDYIE_R = crate::BitReader;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RDYIE` reader - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
pub type PLL1RDYIE_R = crate::BitReader;
///Field `PLL1RDYIE` writer - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
pub type PLL1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2RDYIE` reader - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
pub type PLL2RDYIE_R = crate::BitReader;
///Field `PLL2RDYIE` writer - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
pub type PLL2RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3RDYIE` reader - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
pub type PLL3RDYIE_R = crate::BitReader;
///Field `PLL3RDYIE` writer - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
pub type PLL3RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIKRDYIE` reader - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
pub type MSIKRDYIE_R = crate::BitReader;
///Field `MSIKRDYIE` writer - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
pub type MSIKRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHSIRDYIE` reader - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
pub type SHSIRDYIE_R = crate::BitReader;
///Field `SHSIRDYIE` writer - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
pub type SHSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
    #[inline(always)]
    pub fn msisrdyie(&self) -> MSISRDYIE_R {
        MSISRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
    #[inline(always)]
    pub fn msikrdyie(&self) -> MSIKRDYIE_R {
        MSIKRDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
    #[inline(always)]
    pub fn shsirdyie(&self) -> SHSIRDYIE_R {
        SHSIRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("msisrdyie", &self.msisrdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .field("pll1rdyie", &self.pll1rdyie())
            .field("pll2rdyie", &self.pll2rdyie())
            .field("pll3rdyie", &self.pll3rdyie())
            .field("msikrdyie", &self.msikrdyie())
            .field("shsirdyie", &self.shsirdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<RCC_CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<RCC_CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn msisrdyie(&mut self) -> MSISRDYIE_W<RCC_CIERrs> {
        MSISRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<RCC_CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<RCC_CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 5 - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<RCC_CIERrs> {
        HSI48RDYIE_W::new(self, 5)
    }
    ///Bit 6 - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<RCC_CIERrs> {
        PLL1RDYIE_W::new(self, 6)
    }
    ///Bit 7 - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<RCC_CIERrs> {
        PLL2RDYIE_W::new(self, 7)
    }
    ///Bit 8 - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<RCC_CIERrs> {
        PLL3RDYIE_W::new(self, 8)
    }
    ///Bit 11 - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn msikrdyie(&mut self) -> MSIKRDYIE_W<RCC_CIERrs> {
        MSIKRDYIE_W::new(self, 11)
    }
    ///Bit 12 - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn shsirdyie(&mut self) -> SHSIRDYIE_W<RCC_CIERrs> {
        SHSIRDYIE_W::new(self, 12)
    }
}
/**RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:RCC_CIER)*/
pub struct RCC_CIERrs;
impl crate::RegisterSpec for RCC_CIERrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cier::R`](R) reader structure
impl crate::Readable for RCC_CIERrs {}
///`write(|w| ..)` method takes [`rcc_cier::W`](W) writer structure
impl crate::Writable for RCC_CIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CIER to value 0
impl crate::Resettable for RCC_CIERrs {
    const RESET_VALUE: u32 = 0;
}
