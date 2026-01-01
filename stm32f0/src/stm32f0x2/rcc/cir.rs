///Register `CIR` reader
pub type R = crate::R<CIRrs>;
///Register `CIR` writer
pub type W = crate::W<CIRrs>;
/**LSI Ready Interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR {
    ///0: No clock ready interrupt
    NotInterrupted = 0,
    ///1: Clock ready interrupt
    Interrupted = 1,
}
impl From<LSIRDYFR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI Ready Interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR>;
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYFR {
        match self.bits {
            false => LSIRDYFR::NotInterrupted,
            true => LSIRDYFR::Interrupted,
        }
    }
    ///No clock ready interrupt
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR::NotInterrupted
    }
    ///Clock ready interrupt
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR::Interrupted
    }
}
///Field `LSERDYF` reader - LSE Ready Interrupt flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `HSIRDYF` reader - HSI Ready Interrupt flag
pub use LSIRDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE Ready Interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL Ready Interrupt flag
pub use LSIRDYF_R as PLLRDYF_R;
///Field `HSI14RDYF` reader - HSI14 ready interrupt flag
pub use LSIRDYF_R as HSI14RDYF_R;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub use LSIRDYF_R as HSI48RDYF_R;
/**Clock Security System Interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFR {
    ///0: No clock security interrupt caused by HSE clock failure
    NotInterrupted = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Interrupted = 1,
}
impl From<CSSFR> for bool {
    #[inline(always)]
    fn from(variant: CSSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - Clock Security System Interrupt flag
pub type CSSF_R = crate::BitReader<CSSFR>;
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSFR {
        match self.bits {
            false => CSSFR::NotInterrupted,
            true => CSSFR::Interrupted,
        }
    }
    ///No clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSFR::NotInterrupted
    }
    ///Clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSFR::Interrupted
    }
}
/**LSI Ready Interrupt Enable

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
///Field `LSIRDYIE` reader - LSI Ready Interrupt Enable
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
///Field `LSIRDYIE` writer - LSI Ready Interrupt Enable
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
///Field `LSERDYIE` reader - LSE Ready Interrupt Enable
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSIRDYIE` reader - HSI Ready Interrupt Enable
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE Ready Interrupt Enable
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLL Ready Interrupt Enable
pub use LSIRDYIE_R as PLLRDYIE_R;
///Field `HSI14RDYIE` reader - HSI14 ready interrupt enable
pub use LSIRDYIE_R as HSI14RDYIE_R;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable
pub use LSIRDYIE_R as HSI48RDYIE_R;
///Field `LSERDYIE` writer - LSE Ready Interrupt Enable
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `HSIRDYIE` writer - HSI Ready Interrupt Enable
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE Ready Interrupt Enable
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `PLLRDYIE` writer - PLL Ready Interrupt Enable
pub use LSIRDYIE_W as PLLRDYIE_W;
///Field `HSI14RDYIE` writer - HSI14 ready interrupt enable
pub use LSIRDYIE_W as HSI14RDYIE_W;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable
pub use LSIRDYIE_W as HSI48RDYIE_W;
/**LSI Ready Interrupt Clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSIRDYCW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI Ready Interrupt Clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYCW>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYCW::Clear)
    }
}
///Field `LSERDYC` writer - LSE Ready Interrupt Clear
pub use LSIRDYC_W as LSERDYC_W;
///Field `HSIRDYC` writer - HSI Ready Interrupt Clear
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE Ready Interrupt Clear
pub use LSIRDYC_W as HSERDYC_W;
///Field `PLLRDYC` writer - PLL Ready Interrupt Clear
pub use LSIRDYC_W as PLLRDYC_W;
///Field `HSI14RDYC` writer - HSI 14 MHz Ready Interrupt Clear
pub use LSIRDYC_W as HSI14RDYC_W;
///Field `HSI48RDYC` writer - HSI48 Ready Interrupt Clear
pub use LSIRDYC_W as HSI48RDYC_W;
/**Clock security system interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSCW {
    ///1: Clear CSSF flag
    Clear = 1,
}
impl From<CSSCW> for bool {
    #[inline(always)]
    fn from(variant: CSSCW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - Clock security system interrupt clear
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG, CSSCW>;
impl<'a, REG> CSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear CSSF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSCW::Clear)
    }
}
impl R {
    ///Bit 0 - LSI Ready Interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE Ready Interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI Ready Interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE Ready Interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL Ready Interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI14 ready interrupt flag
    #[inline(always)]
    pub fn hsi14rdyf(&self) -> HSI14RDYF_R {
        HSI14RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clock Security System Interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LSI Ready Interrupt Enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Ready Interrupt Enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI Ready Interrupt Enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSE Ready Interrupt Enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PLL Ready Interrupt Enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HSI14 ready interrupt enable
    #[inline(always)]
    pub fn hsi14rdyie(&self) -> HSI14RDYIE_R {
        HSI14RDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hsi14rdyf", &self.hsi14rdyf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .field("cssf", &self.cssf())
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hsi14rdyie", &self.hsi14rdyie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .finish()
    }
}
impl W {
    ///Bit 8 - LSI Ready Interrupt Enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIRrs> {
        LSIRDYIE_W::new(self, 8)
    }
    ///Bit 9 - LSE Ready Interrupt Enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIRrs> {
        LSERDYIE_W::new(self, 9)
    }
    ///Bit 10 - HSI Ready Interrupt Enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIRrs> {
        HSIRDYIE_W::new(self, 10)
    }
    ///Bit 11 - HSE Ready Interrupt Enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIRrs> {
        HSERDYIE_W::new(self, 11)
    }
    ///Bit 12 - PLL Ready Interrupt Enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIRrs> {
        PLLRDYIE_W::new(self, 12)
    }
    ///Bit 13 - HSI14 ready interrupt enable
    #[inline(always)]
    pub fn hsi14rdyie(&mut self) -> HSI14RDYIE_W<'_, CIRrs> {
        HSI14RDYIE_W::new(self, 13)
    }
    ///Bit 14 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<'_, CIRrs> {
        HSI48RDYIE_W::new(self, 14)
    }
    ///Bit 16 - LSI Ready Interrupt Clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CIRrs> {
        LSIRDYC_W::new(self, 16)
    }
    ///Bit 17 - LSE Ready Interrupt Clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CIRrs> {
        LSERDYC_W::new(self, 17)
    }
    ///Bit 18 - HSI Ready Interrupt Clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CIRrs> {
        HSIRDYC_W::new(self, 18)
    }
    ///Bit 19 - HSE Ready Interrupt Clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CIRrs> {
        HSERDYC_W::new(self, 19)
    }
    ///Bit 20 - PLL Ready Interrupt Clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CIRrs> {
        PLLRDYC_W::new(self, 20)
    }
    ///Bit 21 - HSI 14 MHz Ready Interrupt Clear
    #[inline(always)]
    pub fn hsi14rdyc(&mut self) -> HSI14RDYC_W<'_, CIRrs> {
        HSI14RDYC_W::new(self, 21)
    }
    ///Bit 22 - HSI48 Ready Interrupt Clear
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<'_, CIRrs> {
        HSI48RDYC_W::new(self, 22)
    }
    ///Bit 23 - Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, CIRrs> {
        CSSC_W::new(self, 23)
    }
}
/**Clock interrupt register (RCC_CIR)

You can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RCC:CIR)*/
pub struct CIRrs;
impl crate::RegisterSpec for CIRrs {
    type Ux = u32;
}
///`read()` method returns [`cir::R`](R) reader structure
impl crate::Readable for CIRrs {}
///`write(|w| ..)` method takes [`cir::W`](W) writer structure
impl crate::Writable for CIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIR to value 0
impl crate::Resettable for CIRrs {}
