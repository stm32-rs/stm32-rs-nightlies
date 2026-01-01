///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `LSIONC` writer - LSI oscillator enable in Run/Sleep mode.
pub type LSIONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEONC` writer - LSE oscillator enable in Run/Sleep mode.
pub type LSEONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIONC` writer - MSI oscillator enable in Run/Sleep mode.
pub type MSIONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIONC` writer - HSI oscillator enable in Run/Sleep mode.
pub type HSIONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEONC` writer - HSE oscillator enable in Run/Sleep mode.
pub type HSEONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1ONC` writer - PLL1 oscillator enable in Run/Sleep mode.
pub type PLL1ONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2ONC` writer - PLL2 oscillator enable in Run/Sleep mode.
pub type PLL2ONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3ONC` writer - PLL3 oscillator enable in Run/Sleep mode.
pub type PLL3ONC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4ONC` writer - PLL4 oscillator enable in Run/Sleep mode.
pub type PLL4ONC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lsionc(&mut self) -> LSIONC_W<'_, CCRrs> {
        LSIONC_W::new(self, 0)
    }
    ///Bit 1 - LSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lseonc(&mut self) -> LSEONC_W<'_, CCRrs> {
        LSEONC_W::new(self, 1)
    }
    ///Bit 2 - MSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn msionc(&mut self) -> MSIONC_W<'_, CCRrs> {
        MSIONC_W::new(self, 2)
    }
    ///Bit 3 - HSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hsionc(&mut self) -> HSIONC_W<'_, CCRrs> {
        HSIONC_W::new(self, 3)
    }
    ///Bit 4 - HSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hseonc(&mut self) -> HSEONC_W<'_, CCRrs> {
        HSEONC_W::new(self, 4)
    }
    ///Bit 8 - PLL1 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll1onc(&mut self) -> PLL1ONC_W<'_, CCRrs> {
        PLL1ONC_W::new(self, 8)
    }
    ///Bit 9 - PLL2 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll2onc(&mut self) -> PLL2ONC_W<'_, CCRrs> {
        PLL2ONC_W::new(self, 9)
    }
    ///Bit 10 - PLL3 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll3onc(&mut self) -> PLL3ONC_W<'_, CCRrs> {
        PLL3ONC_W::new(self, 10)
    }
    ///Bit 11 - PLL4 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll4onc(&mut self) -> PLL4ONC_W<'_, CCRrs> {
        PLL4ONC_W::new(self, 11)
    }
}
/**RCC control Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
