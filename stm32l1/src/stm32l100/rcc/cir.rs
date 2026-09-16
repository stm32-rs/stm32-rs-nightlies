///Register `CIR` reader
pub type R = crate::R<CIRrs>;
///Register `CIR` writer
pub type W = crate::W<CIRrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR {
    ///0: Clock is not stable
    NotStable = 0,
    ///1: Clock is stable
    Stable = 1,
}
impl From<LSIRDYFR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR>;
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYFR {
        match self.bits {
            false => LSIRDYFR::NotStable,
            true => LSIRDYFR::Stable,
        }
    }
    ///Clock is not stable
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        *self == LSIRDYFR::NotStable
    }
    ///Clock is stable
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == LSIRDYFR::Stable
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub use LSIRDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub use LSIRDYF_R as PLLRDYF_R;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub use LSIRDYF_R as MSIRDYF_R;
/**LSE Clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSFR {
    ///0: No failure detected on the external 32 KHz oscillator
    NoFailure = 0,
    ///1: A failure is detected on the external 32 kHz oscillator
    Failure = 1,
}
impl From<LSECSSFR> for bool {
    #[inline(always)]
    fn from(variant: LSECSSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub type LSECSSF_R = crate::BitReader<LSECSSFR>;
impl LSECSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSFR {
        match self.bits {
            false => LSECSSFR::NoFailure,
            true => LSECSSFR::Failure,
        }
    }
    ///No failure detected on the external 32 KHz oscillator
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSFR::NoFailure
    }
    ///A failure is detected on the external 32 kHz oscillator
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSFR::Failure
    }
}
///Field `LSECSSF` writer - LSE Clock security system interrupt flag
pub type LSECSSF_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSFR>;
impl<'a, REG> LSECSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No failure detected on the external 32 KHz oscillator
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSFR::NoFailure)
    }
    ///A failure is detected on the external 32 kHz oscillator
    #[inline(always)]
    pub fn failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSFR::Failure)
    }
}
/**Clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFR {
    ///0: No clock security interrupt caused by HSE clock failure
    NotInterupted = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Interupted = 1,
}
impl From<CSSFR> for bool {
    #[inline(always)]
    fn from(variant: CSSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - Clock security system interrupt flag
pub type CSSF_R = crate::BitReader<CSSFR>;
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSFR {
        match self.bits {
            false => CSSFR::NotInterupted,
            true => CSSFR::Interupted,
        }
    }
    ///No clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_not_interupted(&self) -> bool {
        *self == CSSFR::NotInterupted
    }
    ///Clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_interupted(&self) -> bool {
        *self == CSSFR::Interupted
    }
}
/**LSI ready interrupt enable

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
///Field `LSIRDYIE` reader - LSI ready interrupt enable
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
///Field `LSIRDYIE` writer - LSI ready interrupt enable
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
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSIRDYIE` reader - HSI ready interrupt enable
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE ready interrupt enable
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLL ready interrupt enable
pub use LSIRDYIE_R as PLLRDYIE_R;
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub use LSIRDYIE_R as MSIRDYIE_R;
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `HSIRDYIE` writer - HSI ready interrupt enable
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE ready interrupt enable
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `PLLRDYIE` writer - PLL ready interrupt enable
pub use LSIRDYIE_W as PLLRDYIE_W;
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub use LSIRDYIE_W as MSIRDYIE_W;
/**LSE clock security system interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE {
    ///0: LSE CSS interrupt disabled
    Disabled = 0,
    ///1: LSE CSS interrupt enabled
    Enabled = 1,
}
impl From<LSECSSIE> for bool {
    #[inline(always)]
    fn from(variant: LSECSSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSIE` reader - LSE clock security system interrupt enable
pub type LSECSSIE_R = crate::BitReader<LSECSSIE>;
impl LSECSSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSIE {
        match self.bits {
            false => LSECSSIE::Disabled,
            true => LSECSSIE::Enabled,
        }
    }
    ///LSE CSS interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSIE::Disabled
    }
    ///LSE CSS interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSIE::Enabled
    }
}
///Field `LSECSSIE` writer - LSE clock security system interrupt enable
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSIE>;
impl<'a, REG> LSECSSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE CSS interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Disabled)
    }
    ///LSE CSS interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Enabled)
    }
}
/**LSI ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    ///1: Clear interrupt
    Clear = 1,
}
impl From<LSIRDYCW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYCW>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYCW::Clear)
    }
}
///Field `LSERDYC` writer - LSE ready interrupt clear
pub use LSIRDYC_W as LSERDYC_W;
///Field `HSIRDYC` writer - HSI ready interrupt clear
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE ready interrupt clear
pub use LSIRDYC_W as HSERDYC_W;
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub use LSIRDYC_W as PLLRDYC_W;
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub use LSIRDYC_W as MSIRDYC_W;
/**LSE Clock security system interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSCW {
    ///1: Clear interrupt
    Clear = 1,
}
impl From<LSECSSCW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSCW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSCW>;
impl<'a, REG> LSECSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSCW::Clear)
    }
}
///Field `CSSC` writer - Clock security system interrupt clear
pub use LSECSSC_W as CSSC_W;
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIR")
            .field("lsirdyie", &self.lsirdyie())
            .field("msirdyie", &self.msirdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("cssf", &self.cssf())
            .field("lsirdyf", &self.lsirdyf())
            .field("msirdyf", &self.msirdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("lsecssf", &self.lsecssf())
            .field("lsecssie", &self.lsecssie())
            .finish()
    }
}
impl W {
    ///Bit 6 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LSECSSF_W<'_, CIRrs> {
        LSECSSF_W::new(self, 6)
    }
    ///Bit 8 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIRrs> {
        LSIRDYIE_W::new(self, 8)
    }
    ///Bit 9 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIRrs> {
        LSERDYIE_W::new(self, 9)
    }
    ///Bit 10 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIRrs> {
        HSIRDYIE_W::new(self, 10)
    }
    ///Bit 11 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIRrs> {
        HSERDYIE_W::new(self, 11)
    }
    ///Bit 12 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIRrs> {
        PLLRDYIE_W::new(self, 12)
    }
    ///Bit 13 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<'_, CIRrs> {
        MSIRDYIE_W::new(self, 13)
    }
    ///Bit 14 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, CIRrs> {
        LSECSSIE_W::new(self, 14)
    }
    ///Bit 16 - LSI ready interrupt clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CIRrs> {
        LSIRDYC_W::new(self, 16)
    }
    ///Bit 17 - LSE ready interrupt clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CIRrs> {
        LSERDYC_W::new(self, 17)
    }
    ///Bit 18 - HSI ready interrupt clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CIRrs> {
        HSIRDYC_W::new(self, 18)
    }
    ///Bit 19 - HSE ready interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CIRrs> {
        HSERDYC_W::new(self, 19)
    }
    ///Bit 20 - PLL ready interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CIRrs> {
        PLLRDYC_W::new(self, 20)
    }
    ///Bit 21 - MSI ready interrupt clear
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<'_, CIRrs> {
        MSIRDYC_W::new(self, 21)
    }
    ///Bit 22 - LSE Clock security system interrupt clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, CIRrs> {
        LSECSSC_W::new(self, 22)
    }
    ///Bit 23 - Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, CIRrs> {
        CSSC_W::new(self, 23)
    }
}
/**Clock interrupt register

You can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:CIR)*/
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
