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
///Field `PLL1RDYC` writer - PLL1 ready interrupt clear
pub type PLL1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2RDYC` writer - PLL2 ready interrupt clear
pub type PLL2RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3RDYC` writer - PLL3 ready interrupt clear
pub type PLL3RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4RDYC` writer - PLL4 ready interrupt clear
pub type PLL4RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSC` writer - LSE ready interrupt clear
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSC` writer - HSE ready interrupt clear
pub type HSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPFC` writer - CPU Wakeup ready interrupt clear
pub type WKUPFC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - PLL1 ready interrupt clear
    #[inline(always)]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<'_, CICRrs> {
        PLL1RDYC_W::new(self, 8)
    }
    ///Bit 9 - PLL2 ready interrupt clear
    #[inline(always)]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<'_, CICRrs> {
        PLL2RDYC_W::new(self, 9)
    }
    ///Bit 10 - PLL3 ready interrupt clear
    #[inline(always)]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<'_, CICRrs> {
        PLL3RDYC_W::new(self, 10)
    }
    ///Bit 11 - PLL4 ready interrupt clear
    #[inline(always)]
    pub fn pll4rdyc(&mut self) -> PLL4RDYC_W<'_, CICRrs> {
        PLL4RDYC_W::new(self, 11)
    }
    ///Bit 16 - LSE ready interrupt clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, CICRrs> {
        LSECSSC_W::new(self, 16)
    }
    ///Bit 17 - HSE ready interrupt clear
    #[inline(always)]
    pub fn hsecssc(&mut self) -> HSECSSC_W<'_, CICRrs> {
        HSECSSC_W::new(self, 17)
    }
    ///Bit 24 - CPU Wakeup ready interrupt clear
    #[inline(always)]
    pub fn wkupfc(&mut self) -> WKUPFC_W<'_, CICRrs> {
        WKUPFC_W::new(self, 24)
    }
}
/**RCC clock-source interrupt Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CICR)*/
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
