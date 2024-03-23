#[doc = "Register `CIR` reader"]
pub type R = crate::R<CIRrs>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CIRrs>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR {
    #[doc = "0: Clock is not stable"]
    NotStable = 0,
    #[doc = "1: Clock is stable"]
    Stable = 1,
}
impl From<LSIRDYFR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR>;
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYFR {
        match self.bits {
            false => LSIRDYFR::NotStable,
            true => LSIRDYFR::Stable,
        }
    }
    #[doc = "Clock is not stable"]
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        *self == LSIRDYFR::NotStable
    }
    #[doc = "Clock is stable"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == LSIRDYFR::Stable
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub use LSIRDYF_R as HSIRDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub use LSIRDYF_R as PLLRDYF_R;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub use LSIRDYF_R as MSIRDYF_R;
#[doc = "LSE Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSFR {
    #[doc = "0: No failure detected on the external 32 KHz oscillator"]
    NoFailure = 0,
    #[doc = "1: A failure is detected on the external 32 kHz oscillator"]
    Failure = 1,
}
impl From<LSECSSFR> for bool {
    #[inline(always)]
    fn from(variant: LSECSSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LSECSSF_R = crate::BitReader<LSECSSFR>;
impl LSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSFR {
        match self.bits {
            false => LSECSSFR::NoFailure,
            true => LSECSSFR::Failure,
        }
    }
    #[doc = "No failure detected on the external 32 KHz oscillator"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSFR::NoFailure
    }
    #[doc = "A failure is detected on the external 32 kHz oscillator"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSFR::Failure
    }
}
#[doc = "Field `LSECSSF` writer - LSE Clock security system interrupt flag"]
pub type LSECSSF_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSFR>;
impl<'a, REG> LSECSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No failure detected on the external 32 KHz oscillator"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSFR::NoFailure)
    }
    #[doc = "A failure is detected on the external 32 kHz oscillator"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSFR::Failure)
    }
}
#[doc = "Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFR {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NotInterupted = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Interupted = 1,
}
impl From<CSSFR> for bool {
    #[inline(always)]
    fn from(variant: CSSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<CSSFR>;
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSFR {
        match self.bits {
            false => CSSFR::NotInterupted,
            true => CSSFR::Interupted,
        }
    }
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_not_interupted(&self) -> bool {
        *self == CSSFR::NotInterupted
    }
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_interupted(&self) -> bool {
        *self == CSSFR::Interupted
    }
}
#[doc = "LSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub use LSIRDYIE_R as LSERDYIE_R;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub use LSIRDYIE_R as HSIRDYIE_R;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub use LSIRDYIE_R as HSERDYIE_R;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub use LSIRDYIE_R as PLLRDYIE_R;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub use LSIRDYIE_R as MSIRDYIE_R;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub use LSIRDYIE_W as LSERDYIE_W;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub use LSIRDYIE_W as HSIRDYIE_W;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub use LSIRDYIE_W as HSERDYIE_W;
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub use LSIRDYIE_W as PLLRDYIE_W;
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub use LSIRDYIE_W as MSIRDYIE_W;
#[doc = "LSE clock security system interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE {
    #[doc = "0: LSE CSS interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSE CSS interrupt enabled"]
    Enabled = 1,
}
impl From<LSECSSIE> for bool {
    #[inline(always)]
    fn from(variant: LSECSSIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LSECSSIE_R = crate::BitReader<LSECSSIE>;
impl LSECSSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSIE {
        match self.bits {
            false => LSECSSIE::Disabled,
            true => LSECSSIE::Enabled,
        }
    }
    #[doc = "LSE CSS interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSIE::Disabled
    }
    #[doc = "LSE CSS interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSIE::Enabled
    }
}
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable"]
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSIE>;
impl<'a, REG> LSECSSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE CSS interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Disabled)
    }
    #[doc = "LSE CSS interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Enabled)
    }
}
#[doc = "LSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    #[doc = "1: Clear interrupt"]
    Clear = 1,
}
impl From<LSIRDYCW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYCW>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYCW::Clear)
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub use LSIRDYC_W as LSERDYC_W;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub use LSIRDYC_W as HSIRDYC_W;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub use LSIRDYC_W as HSERDYC_W;
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub use LSIRDYC_W as PLLRDYC_W;
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub use LSIRDYC_W as MSIRDYC_W;
#[doc = "LSE Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSCW {
    #[doc = "1: Clear interrupt"]
    Clear = 1,
}
impl From<LSECSSCW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSCW>;
impl<'a, REG> LSECSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSCW::Clear)
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub use LSECSSC_W as CSSC_W;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssf(&mut self) -> LSECSSF_W<CIRrs> {
        LSECSSF_W::new(self, 6)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIRrs> {
        LSIRDYIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIRrs> {
        LSERDYIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIRrs> {
        HSIRDYIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIRrs> {
        HSERDYIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIRrs> {
        PLLRDYIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - MSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<CIRrs> {
        MSIRDYIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<CIRrs> {
        LSECSSIE_W::new(self, 14)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CIRrs> {
        LSIRDYC_W::new(self, 16)
    }
    #[doc = "Bit 17 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CIRrs> {
        LSERDYC_W::new(self, 17)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CIRrs> {
        HSIRDYC_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CIRrs> {
        HSERDYC_W::new(self, 19)
    }
    #[doc = "Bit 20 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CIRrs> {
        PLLRDYC_W::new(self, 20)
    }
    #[doc = "Bit 21 - MSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<CIRrs> {
        MSIRDYC_W::new(self, 21)
    }
    #[doc = "Bit 22 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CIRrs> {
        LSECSSC_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CIRrs> {
        CSSC_W::new(self, 23)
    }
}
#[doc = "Clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIRrs;
impl crate::RegisterSpec for CIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CIRrs {}
#[doc = "`write(|w| ..)` method takes [`cir::W`](W) writer structure"]
impl crate::Writable for CIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIRrs {
    const RESET_VALUE: u32 = 0;
}
