///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LSION` reader - LSI oscillator enable in Run/Sleep mode.
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - LSI oscillator enable in Run/Sleep mode.
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEON` reader - LSE oscillator enable in Run/Sleep mode.
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - LSE oscillator enable in Run/Sleep mode.
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSION` reader - MSI oscillator enable in Run/Sleep mode.
pub type MSION_R = crate::BitReader;
///Field `MSION` writer - MSI oscillator enable in Run/Sleep mode.
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSION` reader - HSI oscillator enable in Run/Sleep mode.
pub type HSION_R = crate::BitReader;
///Field `HSION` writer - HSI oscillator enable in Run/Sleep mode.
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEON` reader - HSE oscillator enable in Run/Sleep mode.
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - HSE oscillator enable in Run/Sleep mode.
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1ON` reader - PLL1 enable in Run/Sleep mode.
pub type PLL1ON_R = crate::BitReader;
///Field `PLL1ON` writer - PLL1 enable in Run/Sleep mode.
pub type PLL1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2ON` reader - PLL2 enable in Run/Sleep mode.
pub type PLL2ON_R = crate::BitReader;
///Field `PLL2ON` writer - PLL2 enable in Run/Sleep mode.
pub type PLL2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3ON` reader - PLL3 enable in Run/Sleep mode.
pub type PLL3ON_R = crate::BitReader;
///Field `PLL3ON` writer - PLL3 enable in Run/Sleep mode.
pub type PLL3ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4ON` reader - PLL4 enable in Run/Sleep mode.
pub type PLL4ON_R = crate::BitReader;
///Field `PLL4ON` writer - PLL4 enable in Run/Sleep mode.
pub type PLL4ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll4on(&self) -> PLL4ON_R {
        PLL4ON_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lsion", &self.lsion())
            .field("lseon", &self.lseon())
            .field("msion", &self.msion())
            .field("hsion", &self.hsion())
            .field("hseon", &self.hseon())
            .field("pll1on", &self.pll1on())
            .field("pll2on", &self.pll2on())
            .field("pll3on", &self.pll3on())
            .field("pll4on", &self.pll4on())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 1 - LSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, CRrs> {
        LSEON_W::new(self, 1)
    }
    ///Bit 2 - MSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<'_, CRrs> {
        MSION_W::new(self, 2)
    }
    ///Bit 3 - HSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 3)
    }
    ///Bit 4 - HSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 4)
    }
    ///Bit 8 - PLL1 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll1on(&mut self) -> PLL1ON_W<'_, CRrs> {
        PLL1ON_W::new(self, 8)
    }
    ///Bit 9 - PLL2 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll2on(&mut self) -> PLL2ON_W<'_, CRrs> {
        PLL2ON_W::new(self, 9)
    }
    ///Bit 10 - PLL3 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll3on(&mut self) -> PLL3ON_W<'_, CRrs> {
        PLL3ON_W::new(self, 10)
    }
    ///Bit 11 - PLL4 enable in Run/Sleep mode.
    #[inline(always)]
    pub fn pll4on(&mut self) -> PLL4ON_W<'_, CRrs> {
        PLL4ON_W::new(self, 11)
    }
}
/**RCC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x08
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x08;
}
