///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
/**LSI ready Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready Interrupt Enable
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
///Field `LSIRDYIE` writer - LSI ready Interrupt Enable
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
///Field `LSERDYIE` reader - LSE ready Interrupt Enable
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSIRDYIE` reader - HSI ready Interrupt Enable
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE ready Interrupt Enable
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `CSIRDYIE` reader - CSI ready Interrupt Enable
pub use LSIRDYIE_R as CSIRDYIE_R;
///Field `HSI48RDYIE` reader - RC48 ready Interrupt Enable
pub use LSIRDYIE_R as HSI48RDYIE_R;
///Field `PLL1RDYIE` reader - PLL1 ready Interrupt Enable
pub use LSIRDYIE_R as PLL1RDYIE_R;
///Field `PLL2RDYIE` reader - PLL2 ready Interrupt Enable
pub use LSIRDYIE_R as PLL2RDYIE_R;
///Field `PLL3RDYIE` reader - PLL3 ready Interrupt Enable
pub use LSIRDYIE_R as PLL3RDYIE_R;
///Field `LSECSSIE` reader - LSE clock security system Interrupt Enable
pub use LSIRDYIE_R as LSECSSIE_R;
///Field `LSERDYIE` writer - LSE ready Interrupt Enable
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `HSIRDYIE` writer - HSI ready Interrupt Enable
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE ready Interrupt Enable
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `CSIRDYIE` writer - CSI ready Interrupt Enable
pub use LSIRDYIE_W as CSIRDYIE_W;
///Field `HSI48RDYIE` writer - RC48 ready Interrupt Enable
pub use LSIRDYIE_W as HSI48RDYIE_W;
///Field `PLL1RDYIE` writer - PLL1 ready Interrupt Enable
pub use LSIRDYIE_W as PLL1RDYIE_W;
///Field `PLL2RDYIE` writer - PLL2 ready Interrupt Enable
pub use LSIRDYIE_W as PLL2RDYIE_W;
///Field `PLL3RDYIE` writer - PLL3 ready Interrupt Enable
pub use LSIRDYIE_W as PLL3RDYIE_W;
///Field `LSECSSIE` writer - LSE clock security system Interrupt Enable
pub use LSIRDYIE_W as LSECSSIE_W;
impl R {
    ///Bit 0 - LSI ready Interrupt Enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready Interrupt Enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready Interrupt Enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready Interrupt Enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSI ready Interrupt Enable
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RC48 ready Interrupt Enable
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready Interrupt Enable
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready Interrupt Enable
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready Interrupt Enable
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system Interrupt Enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("csirdyie", &self.csirdyie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .field("pll1rdyie", &self.pll1rdyie())
            .field("pll2rdyie", &self.pll2rdyie())
            .field("pll3rdyie", &self.pll3rdyie())
            .field("lsecssie", &self.lsecssie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready Interrupt Enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready Interrupt Enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - HSI ready Interrupt Enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIERrs> {
        HSIRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSE ready Interrupt Enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 3)
    }
    ///Bit 4 - CSI ready Interrupt Enable
    #[inline(always)]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<'_, CIERrs> {
        CSIRDYIE_W::new(self, 4)
    }
    ///Bit 5 - RC48 ready Interrupt Enable
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<'_, CIERrs> {
        HSI48RDYIE_W::new(self, 5)
    }
    ///Bit 6 - PLL1 ready Interrupt Enable
    #[inline(always)]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<'_, CIERrs> {
        PLL1RDYIE_W::new(self, 6)
    }
    ///Bit 7 - PLL2 ready Interrupt Enable
    #[inline(always)]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<'_, CIERrs> {
        PLL2RDYIE_W::new(self, 7)
    }
    ///Bit 8 - PLL3 ready Interrupt Enable
    #[inline(always)]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<'_, CIERrs> {
        PLL3RDYIE_W::new(self, 8)
    }
    ///Bit 9 - LSE clock security system Interrupt Enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
}
/**RCC Clock Source Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RCC:CIER)*/
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
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
