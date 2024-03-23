#[doc = "Register `CIR` reader"]
pub type R = crate::R<CIRrs>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CIRrs>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR {
    #[doc = "0: No clock ready interrupt"]
    NotInterrupted = 0,
    #[doc = "1: Clock ready interrupt"]
    Interrupted = 1,
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
            false => LSIRDYFR::NotInterrupted,
            true => LSIRDYFR::Interrupted,
        }
    }
    #[doc = "No clock ready interrupt"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR::NotInterrupted
    }
    #[doc = "Clock ready interrupt"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR::Interrupted
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub use LSIRDYF_R as HSIRDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `PLLRDYF` reader - Main PLL (PLL) ready interrupt flag"]
pub use LSIRDYF_R as PLLRDYF_R;
#[doc = "Field `PLLI2SRDYF` reader - PLLI2S ready interrupt flag"]
pub use LSIRDYF_R as PLLI2SRDYF_R;
#[doc = "Field `PLLSAIRDYF` reader - PLLSAI ready interrupt flag"]
pub use LSIRDYF_R as PLLSAIRDYF_R;
#[doc = "Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFR {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NotInterrupted = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Interrupted = 1,
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
            false => CSSFR::NotInterrupted,
            true => CSSFR::Interrupted,
        }
    }
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSFR::NotInterrupted
    }
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSFR::Interrupted
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
#[doc = "Field `PLLRDYIE` reader - Main PLL (PLL) ready interrupt enable"]
pub use LSIRDYIE_R as PLLRDYIE_R;
#[doc = "Field `PLLI2SRDYIE` reader - PLLI2S ready interrupt enable"]
pub use LSIRDYIE_R as PLLI2SRDYIE_R;
#[doc = "Field `PLLSAIRDYIE` reader - PLLSAI Ready Interrupt Enable"]
pub use LSIRDYIE_R as PLLSAIRDYIE_R;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub use LSIRDYIE_W as LSERDYIE_W;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub use LSIRDYIE_W as HSIRDYIE_W;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub use LSIRDYIE_W as HSERDYIE_W;
#[doc = "Field `PLLRDYIE` writer - Main PLL (PLL) ready interrupt enable"]
pub use LSIRDYIE_W as PLLRDYIE_W;
#[doc = "Field `PLLI2SRDYIE` writer - PLLI2S ready interrupt enable"]
pub use LSIRDYIE_W as PLLI2SRDYIE_W;
#[doc = "Field `PLLSAIRDYIE` writer - PLLSAI Ready Interrupt Enable"]
pub use LSIRDYIE_W as PLLSAIRDYIE_W;
#[doc = "LSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    #[doc = "1: Clear interrupt flag"]
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
    #[doc = "Clear interrupt flag"]
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
#[doc = "Field `PLLRDYC` writer - Main PLL(PLL) ready interrupt clear"]
pub use LSIRDYC_W as PLLRDYC_W;
#[doc = "Field `PLLI2SRDYC` writer - PLLI2S ready interrupt clear"]
pub use LSIRDYC_W as PLLI2SRDYC_W;
#[doc = "Field `PLLSAIRDYC` writer - PLLSAI Ready Interrupt Clear"]
pub use LSIRDYC_W as PLLSAIRDYC_W;
#[doc = "Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSCW {
    #[doc = "1: Clear CSSF flag"]
    Clear = 1,
}
impl From<CSSCW> for bool {
    #[inline(always)]
    fn from(variant: CSSCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG, CSSCW>;
impl<'a, REG> CSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSCW::Clear)
    }
}
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
    #[doc = "Bit 4 - Main PLL (PLL) ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLLI2S ready interrupt flag"]
    #[inline(always)]
    pub fn plli2srdyf(&self) -> PLLI2SRDYF_R {
        PLLI2SRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI ready interrupt flag"]
    #[inline(always)]
    pub fn pllsairdyf(&self) -> PLLSAIRDYF_R {
        PLLSAIRDYF_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLLI2S ready interrupt enable"]
    #[inline(always)]
    pub fn plli2srdyie(&self) -> PLLI2SRDYIE_R {
        PLLI2SRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLSAI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllsairdyie(&self) -> PLLSAIRDYIE_R {
        PLLSAIRDYIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIRrs> {
        PLLRDYIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - PLLI2S ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn plli2srdyie(&mut self) -> PLLI2SRDYIE_W<CIRrs> {
        PLLI2SRDYIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - PLLSAI Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsairdyie(&mut self) -> PLLSAIRDYIE_W<CIRrs> {
        PLLSAIRDYIE_W::new(self, 14)
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
    #[doc = "Bit 20 - Main PLL(PLL) ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CIRrs> {
        PLLRDYC_W::new(self, 20)
    }
    #[doc = "Bit 21 - PLLI2S ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn plli2srdyc(&mut self) -> PLLI2SRDYC_W<CIRrs> {
        PLLI2SRDYC_W::new(self, 21)
    }
    #[doc = "Bit 22 - PLLSAI Ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsairdyc(&mut self) -> PLLSAIRDYC_W<CIRrs> {
        PLLSAIRDYC_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CIRrs> {
        CSSC_W::new(self, 23)
    }
}
#[doc = "clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
