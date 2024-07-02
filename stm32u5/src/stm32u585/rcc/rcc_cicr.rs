///Register `RCC_CICR` writer
pub type W = crate::W<RCC_CICRrs>;
///Field `LSIRDYC` writer - LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYC` writer - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISRDYC` writer - MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
pub type MSISRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYC` writer - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYC` writer - HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDYC` writer - HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RDYC` writer - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
pub type PLL1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2RDYC` writer - PLL2 ready interrupt clear Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
pub type PLL2RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3RDYC` writer - PLL3 ready interrupt clear Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
pub type PLL3RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSC` writer - Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIKRDYC` writer - MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
pub type MSIKRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHSIRDYC` writer - SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
pub type SHSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RCC_CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<RCC_CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<RCC_CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn msisrdyc(&mut self) -> MSISRDYC_W<RCC_CICRrs> {
        MSISRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<RCC_CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<RCC_CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<RCC_CICRrs> {
        HSI48RDYC_W::new(self, 5)
    }
    ///Bit 6 - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<RCC_CICRrs> {
        PLL1RDYC_W::new(self, 6)
    }
    ///Bit 7 - PLL2 ready interrupt clear Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<RCC_CICRrs> {
        PLL2RDYC_W::new(self, 7)
    }
    ///Bit 8 - PLL3 ready interrupt clear Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<RCC_CICRrs> {
        PLL3RDYC_W::new(self, 8)
    }
    ///Bit 10 - Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<RCC_CICRrs> {
        CSSC_W::new(self, 10)
    }
    ///Bit 11 - MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn msikrdyc(&mut self) -> MSIKRDYC_W<RCC_CICRrs> {
        MSIKRDYC_W::new(self, 11)
    }
    ///Bit 12 - SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn shsirdyc(&mut self) -> SHSIRDYC_W<RCC_CICRrs> {
        SHSIRDYC_W::new(self, 12)
    }
}
/**RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:RCC_CICR)*/
pub struct RCC_CICRrs;
impl crate::RegisterSpec for RCC_CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rcc_cicr::W`](W) writer structure
impl crate::Writable for RCC_CICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CICR to value 0
impl crate::Resettable for RCC_CICRrs {
    const RESET_VALUE: u32 = 0;
}
