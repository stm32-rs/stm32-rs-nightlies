///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `LSIONS` writer - LSI oscillator enable in Run/Sleep mode.
pub type LSIONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEONS` writer - LSE oscillator enable in Run/Sleep mode.
pub type LSEONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIONS` writer - MSI oscillator enable in Run/Sleep mode.
pub type MSIONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIONS` writer - HSI oscillator enable in Run/Sleep mode.
pub type HSIONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEONS` writer - HSE oscillator enable in Run/Sleep mode.
pub type HSEONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1ONS` writer - PLL1 oscillator enable in Run/Sleep mode.
pub type PLL1ONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2ONS` writer - PLL2 oscillator enable in Run/Sleep mode.
pub type PLL2ONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3ONS` writer - PLL3 oscillator enable in Run/Sleep mode.
pub type PLL3ONS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4ONS` writer - PLL4 oscillator enable in Run/Sleep mode.
pub type PLL4ONS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lsions(&mut self) -> LSIONS_W<'_, CSRrs> {
        LSIONS_W::new(self, 0)
    }
    ///Bit 1 - LSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lseons(&mut self) -> LSEONS_W<'_, CSRrs> {
        LSEONS_W::new(self, 1)
    }
    ///Bit 2 - MSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn msions(&mut self) -> MSIONS_W<'_, CSRrs> {
        MSIONS_W::new(self, 2)
    }
    ///Bit 3 - HSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hsions(&mut self) -> HSIONS_W<'_, CSRrs> {
        HSIONS_W::new(self, 3)
    }
    ///Bit 4 - HSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hseons(&mut self) -> HSEONS_W<'_, CSRrs> {
        HSEONS_W::new(self, 4)
    }
    ///Bit 8 - PLL1 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll1ons(&mut self) -> PLL1ONS_W<'_, CSRrs> {
        PLL1ONS_W::new(self, 8)
    }
    ///Bit 9 - PLL2 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll2ons(&mut self) -> PLL2ONS_W<'_, CSRrs> {
        PLL2ONS_W::new(self, 9)
    }
    ///Bit 10 - PLL3 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll3ons(&mut self) -> PLL3ONS_W<'_, CSRrs> {
        PLL3ONS_W::new(self, 10)
    }
    ///Bit 11 - PLL4 oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll4ons(&mut self) -> PLL4ONS_W<'_, CSRrs> {
        PLL4ONS_W::new(self, 11)
    }
}
/**RCC control set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
