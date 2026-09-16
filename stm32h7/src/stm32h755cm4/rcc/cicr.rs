///Register `CICR` reader
pub type R = crate::R<CICRrs>;
///Register `CICR` writer
pub type W = crate::W<CICRrs>;
/**LSI ready Interrupt Clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` reader - LSI ready Interrupt Clear
pub type LSIRDYC_R = crate::BitReader<LSIRDYC>;
impl LSIRDYC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LSIRDYC> {
        match self.bits {
            true => Some(LSIRDYC::Clear),
            _ => None,
        }
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LSIRDYC::Clear
    }
}
///Field `LSIRDYC` writer - LSI ready Interrupt Clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYC>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC::Clear)
    }
}
///Field `LSERDYC` reader - LSE ready Interrupt Clear
pub use LSIRDYC_R as LSERDYC_R;
///Field `HSIRDYC` reader - HSI ready Interrupt Clear
pub use LSIRDYC_R as HSIRDYC_R;
///Field `HSERDYC` reader - HSE ready Interrupt Clear
pub use LSIRDYC_R as HSERDYC_R;
///Field `LSERDYC` writer - LSE ready Interrupt Clear
pub use LSIRDYC_W as LSERDYC_W;
///Field `HSIRDYC` writer - HSI ready Interrupt Clear
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE ready Interrupt Clear
pub use LSIRDYC_W as HSERDYC_W;
///Field `HSE_ready_Interrupt_Clear` reader - CSI ready Interrupt Clear
pub type HSE_READY_INTERRUPT_CLEAR_R = crate::BitReader;
///Field `HSE_ready_Interrupt_Clear` writer - CSI ready Interrupt Clear
pub type HSE_READY_INTERRUPT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDYC` reader - RC48 ready Interrupt Clear
pub use LSIRDYC_R as HSI48RDYC_R;
///Field `PLL1RDYC` reader - PLL1 ready Interrupt Clear
pub use LSIRDYC_R as PLL1RDYC_R;
///Field `PLL2RDYC` reader - PLL2 ready Interrupt Clear
pub use LSIRDYC_R as PLL2RDYC_R;
///Field `PLL3RDYC` reader - PLL3 ready Interrupt Clear
pub use LSIRDYC_R as PLL3RDYC_R;
///Field `LSECSSC` reader - LSE clock security system Interrupt Clear
pub use LSIRDYC_R as LSECSSC_R;
///Field `HSECSSC` reader - HSE clock security system Interrupt Clear
pub use LSIRDYC_R as HSECSSC_R;
///Field `HSI48RDYC` writer - RC48 ready Interrupt Clear
pub use LSIRDYC_W as HSI48RDYC_W;
///Field `PLL1RDYC` writer - PLL1 ready Interrupt Clear
pub use LSIRDYC_W as PLL1RDYC_W;
///Field `PLL2RDYC` writer - PLL2 ready Interrupt Clear
pub use LSIRDYC_W as PLL2RDYC_W;
///Field `PLL3RDYC` writer - PLL3 ready Interrupt Clear
pub use LSIRDYC_W as PLL3RDYC_W;
///Field `LSECSSC` writer - LSE clock security system Interrupt Clear
pub use LSIRDYC_W as LSECSSC_W;
///Field `HSECSSC` writer - HSE clock security system Interrupt Clear
pub use LSIRDYC_W as HSECSSC_W;
impl R {
    ///Bit 0 - LSI ready Interrupt Clear
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready Interrupt Clear
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready Interrupt Clear
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready Interrupt Clear
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSI ready Interrupt Clear
    #[inline(always)]
    pub fn hse_ready_interrupt_clear(&self) -> HSE_READY_INTERRUPT_CLEAR_R {
        HSE_READY_INTERRUPT_CLEAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RC48 ready Interrupt Clear
    #[inline(always)]
    pub fn hsi48rdyc(&self) -> HSI48RDYC_R {
        HSI48RDYC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready Interrupt Clear
    #[inline(always)]
    pub fn pll1rdyc(&self) -> PLL1RDYC_R {
        PLL1RDYC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready Interrupt Clear
    #[inline(always)]
    pub fn pll2rdyc(&self) -> PLL2RDYC_R {
        PLL2RDYC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready Interrupt Clear
    #[inline(always)]
    pub fn pll3rdyc(&self) -> PLL3RDYC_R {
        PLL3RDYC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system Interrupt Clear
    #[inline(always)]
    pub fn lsecssc(&self) -> LSECSSC_R {
        LSECSSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSE clock security system Interrupt Clear
    #[inline(always)]
    pub fn hsecssc(&self) -> HSECSSC_R {
        HSECSSC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CICR")
            .field("lsirdyc", &self.lsirdyc())
            .field("lserdyc", &self.lserdyc())
            .field("hsirdyc", &self.hsirdyc())
            .field("hserdyc", &self.hserdyc())
            .field(
                "hse_ready_interrupt_clear",
                &self.hse_ready_interrupt_clear(),
            )
            .field("hsi48rdyc", &self.hsi48rdyc())
            .field("pll1rdyc", &self.pll1rdyc())
            .field("pll2rdyc", &self.pll2rdyc())
            .field("pll3rdyc", &self.pll3rdyc())
            .field("lsecssc", &self.lsecssc())
            .field("hsecssc", &self.hsecssc())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready Interrupt Clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready Interrupt Clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - HSI ready Interrupt Clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CICRrs> {
        HSIRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSE ready Interrupt Clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CICRrs> {
        HSERDYC_W::new(self, 3)
    }
    ///Bit 4 - CSI ready Interrupt Clear
    #[inline(always)]
    pub fn hse_ready_interrupt_clear(&mut self) -> HSE_READY_INTERRUPT_CLEAR_W<'_, CICRrs> {
        HSE_READY_INTERRUPT_CLEAR_W::new(self, 4)
    }
    ///Bit 5 - RC48 ready Interrupt Clear
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<'_, CICRrs> {
        HSI48RDYC_W::new(self, 5)
    }
    ///Bit 6 - PLL1 ready Interrupt Clear
    #[inline(always)]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<'_, CICRrs> {
        PLL1RDYC_W::new(self, 6)
    }
    ///Bit 7 - PLL2 ready Interrupt Clear
    #[inline(always)]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<'_, CICRrs> {
        PLL2RDYC_W::new(self, 7)
    }
    ///Bit 8 - PLL3 ready Interrupt Clear
    #[inline(always)]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<'_, CICRrs> {
        PLL3RDYC_W::new(self, 8)
    }
    ///Bit 9 - LSE clock security system Interrupt Clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, CICRrs> {
        LSECSSC_W::new(self, 9)
    }
    ///Bit 10 - HSE clock security system Interrupt Clear
    #[inline(always)]
    pub fn hsecssc(&mut self) -> HSECSSC_W<'_, CICRrs> {
        HSECSSC_W::new(self, 10)
    }
}
/**RCC Clock Source Interrupt Clear Register

You can [`read`](crate::Reg::read) this register and get [`cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`read()` method returns [`cicr::R`](R) reader structure
impl crate::Readable for CICRrs {}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {}
