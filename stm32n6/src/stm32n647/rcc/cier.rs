///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
///Field `LSIRDYIE` reader - LSI ready interrupt enable
pub type LSIRDYIE_R = crate::BitReader;
///Field `LSIRDYIE` writer - LSI ready interrupt enable
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub type MSIRDYIE_R = crate::BitReader;
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSI ready interrupt enable
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSI ready interrupt enable
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSE ready interrupt enable
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSE ready interrupt enable
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RDYIE` reader - PLL1 ready interrupt enable
pub type PLL1RDYIE_R = crate::BitReader;
///Field `PLL1RDYIE` writer - PLL1 ready interrupt enable
pub type PLL1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2RDYIE` reader - PLL2 ready interrupt enable
pub type PLL2RDYIE_R = crate::BitReader;
///Field `PLL2RDYIE` writer - PLL2 ready interrupt enable
pub type PLL2RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3RDYIE` reader - PLL3 ready interrupt enable
pub type PLL3RDYIE_R = crate::BitReader;
///Field `PLL3RDYIE` writer - PLL3 ready interrupt enable
pub type PLL3RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4RDYIE` reader - PLL4 ready interrupt enable
pub type PLL4RDYIE_R = crate::BitReader;
///Field `PLL4RDYIE` writer - PLL4 ready interrupt enable
pub type PLL4RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSIE` reader - LSE clock security system (CSS) interrupt enable
pub type LSECSSIE_R = crate::BitReader;
///Field `LSECSSIE` writer - LSE clock security system (CSS) interrupt enable
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSIE` reader - HSE clock security system (CSS) interrupt enable
pub type HSECSSIE_R = crate::BitReader;
///Field `HSECSSIE` writer - HSE clock security system (CSS) interrupt enable
pub type HSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPIE` reader - CPU wakeup from Stop interrupt enable
pub type WKUPIE_R = crate::BitReader;
///Field `WKUPIE` writer - CPU wakeup from Stop interrupt enable
pub type WKUPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1 ready interrupt enable
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2 ready interrupt enable
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3 ready interrupt enable
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4 ready interrupt enable
    #[inline(always)]
    pub fn pll4rdyie(&self) -> PLL4RDYIE_R {
        PLL4RDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - LSE clock security system (CSS) interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock security system (CSS) interrupt enable
    #[inline(always)]
    pub fn hsecssie(&self) -> HSECSSIE_R {
        HSECSSIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - CPU wakeup from Stop interrupt enable
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("msirdyie", &self.msirdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("pll1rdyie", &self.pll1rdyie())
            .field("pll2rdyie", &self.pll2rdyie())
            .field("pll3rdyie", &self.pll3rdyie())
            .field("pll4rdyie", &self.pll4rdyie())
            .field("lsecssie", &self.lsecssie())
            .field("hsecssie", &self.hsecssie())
            .field("wkupie", &self.wkupie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<'_, CIERrs> {
        MSIRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 8 - PLL1 ready interrupt enable
    #[inline(always)]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<'_, CIERrs> {
        PLL1RDYIE_W::new(self, 8)
    }
    ///Bit 9 - PLL2 ready interrupt enable
    #[inline(always)]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<'_, CIERrs> {
        PLL2RDYIE_W::new(self, 9)
    }
    ///Bit 10 - PLL3 ready interrupt enable
    #[inline(always)]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<'_, CIERrs> {
        PLL3RDYIE_W::new(self, 10)
    }
    ///Bit 11 - PLL4 ready interrupt enable
    #[inline(always)]
    pub fn pll4rdyie(&mut self) -> PLL4RDYIE_W<'_, CIERrs> {
        PLL4RDYIE_W::new(self, 11)
    }
    ///Bit 16 - LSE clock security system (CSS) interrupt enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, CIERrs> {
        LSECSSIE_W::new(self, 16)
    }
    ///Bit 17 - HSE clock security system (CSS) interrupt enable
    #[inline(always)]
    pub fn hsecssie(&mut self) -> HSECSSIE_W<'_, CIERrs> {
        HSECSSIE_W::new(self, 17)
    }
    ///Bit 24 - CPU wakeup from Stop interrupt enable
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W<'_, CIERrs> {
        WKUPIE_W::new(self, 24)
    }
}
/**RCC clock-source interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`write(|w| ..)` method takes [`cier::W`](W) writer structure
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIER to value 0x0002_0000
impl crate::Resettable for CIERrs {
    const RESET_VALUE: u32 = 0x0002_0000;
}
