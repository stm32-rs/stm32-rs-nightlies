///Register `CICR` writer
pub type W = crate::W<CICRrs>;
///Field `LSIRDYC` writer - LSI ready interrupt clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYC` writer - LSE ready interrupt clear
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYC` writer - HSI ready interrupt clear
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYC` writer - HSE ready interrupt clear
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear
pub type PLLSAI1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2RDYC` writer - PLLSAI2 ready interrupt clear
pub type PLLSAI2RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSC` writer - Clock security system interrupt clear
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt clear
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<'_, CICRrs> {
        MSIRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI ready interrupt clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    ///Bit 6 - PLLSAI1 ready interrupt clear
    #[inline(always)]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<'_, CICRrs> {
        PLLSAI1RDYC_W::new(self, 6)
    }
    ///Bit 7 - PLLSAI2 ready interrupt clear
    #[inline(always)]
    pub fn pllsai2rdyc(&mut self) -> PLLSAI2RDYC_W<'_, CICRrs> {
        PLLSAI2RDYC_W::new(self, 7)
    }
    ///Bit 8 - Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, CICRrs> {
        LSECSSC_W::new(self, 9)
    }
    ///Bit 10 - HSI48 oscillator ready interrupt clear
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<'_, CICRrs> {
        HSI48RDYC_W::new(self, 10)
    }
}
/**Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {}
