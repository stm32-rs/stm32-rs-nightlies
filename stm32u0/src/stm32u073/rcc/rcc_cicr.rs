///Register `RCC_CICR` writer
pub type W = crate::W<RCC_CICRrs>;
///Field `LSIRDYC` writer - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYC` writer - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDYC` writer - MSI ready interrupt clear This bit is set by software to clear the MSIRDYF flag.
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYC` writer - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag.
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYC` writer - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDYC` writer - PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag.
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSC` writer - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSC` writer - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag.
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RCC_CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<RCC_CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<RCC_CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt clear This bit is set by software to clear the MSIRDYF flag.
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<RCC_CICRrs> {
        MSIRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag.
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<RCC_CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<RCC_CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag.
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<RCC_CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    ///Bit 8 - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<RCC_CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<RCC_CICRrs> {
        LSECSSC_W::new(self, 9)
    }
    ///Bit 10 - HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag.
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<RCC_CICRrs> {
        HSI48RDYC_W::new(self, 10)
    }
}
/**Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RCC:RCC_CICR)*/
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