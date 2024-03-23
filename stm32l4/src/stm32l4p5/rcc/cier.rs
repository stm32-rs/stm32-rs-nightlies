#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIERrs>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIERrs>;
#[doc = "LSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    #[doc = "0: LSI ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSI ready interrupt enabled"]
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
    #[doc = "LSI ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    #[doc = "LSI ready interrupt enabled"]
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
    #[doc = "LSI ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
#[doc = "LSE ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE {
    #[doc = "0: LSE ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSE ready interrupt enabled"]
    Enabled = 1,
}
impl From<LSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LSERDYIE_R = crate::BitReader<LSERDYIE>;
impl LSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYIE {
        match self.bits {
            false => LSERDYIE::Disabled,
            true => LSERDYIE::Enabled,
        }
    }
    #[doc = "LSE ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSERDYIE::Disabled
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSERDYIE::Enabled
    }
}
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYIE>;
impl<'a, REG> LSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::Disabled)
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::Enabled)
    }
}
#[doc = "MSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYIE {
    #[doc = "0: MSI ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: MSI ready interrupt enabled"]
    Enabled = 1,
}
impl From<MSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub type MSIRDYIE_R = crate::BitReader<MSIRDYIE>;
impl MSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYIE {
        match self.bits {
            false => MSIRDYIE::Disabled,
            true => MSIRDYIE::Enabled,
        }
    }
    #[doc = "MSI ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIRDYIE::Disabled
    }
    #[doc = "MSI ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIRDYIE::Enabled
    }
}
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, MSIRDYIE>;
impl<'a, REG> MSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYIE::Disabled)
    }
    #[doc = "MSI ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYIE::Enabled)
    }
}
#[doc = "HSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE {
    #[doc = "0: HSI16 ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HSI16 ready interrupt enabled"]
    Enabled = 1,
}
impl From<HSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE>;
impl HSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYIE {
        match self.bits {
            false => HSIRDYIE::Disabled,
            true => HSIRDYIE::Enabled,
        }
    }
    #[doc = "HSI16 ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIRDYIE::Disabled
    }
    #[doc = "HSI16 ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIRDYIE::Enabled
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYIE>;
impl<'a, REG> HSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI16 ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::Disabled)
    }
    #[doc = "HSI16 ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::Enabled)
    }
}
#[doc = "HSE ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE {
    #[doc = "0: HSE ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HSE ready interrupt enabled"]
    Enabled = 1,
}
impl From<HSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader<HSERDYIE>;
impl HSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYIE {
        match self.bits {
            false => HSERDYIE::Disabled,
            true => HSERDYIE::Enabled,
        }
    }
    #[doc = "HSE ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSERDYIE::Disabled
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSERDYIE::Enabled
    }
}
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYIE>;
impl<'a, REG> HSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::Disabled)
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::Enabled)
    }
}
#[doc = "PLL ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYIE {
    #[doc = "0: PLL lock interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PLL lock interrupt enabled"]
    Enabled = 1,
}
impl From<PLLRDYIE> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub type PLLRDYIE_R = crate::BitReader<PLLRDYIE>;
impl PLLRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDYIE {
        match self.bits {
            false => PLLRDYIE::Disabled,
            true => PLLRDYIE::Enabled,
        }
    }
    #[doc = "PLL lock interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLRDYIE::Disabled
    }
    #[doc = "PLL lock interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLRDYIE::Enabled
    }
}
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub type PLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLRDYIE>;
impl<'a, REG> PLLRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL lock interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYIE::Disabled)
    }
    #[doc = "PLL lock interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYIE::Enabled)
    }
}
#[doc = "PLLSAI1 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYIE {
    #[doc = "0: PLLSAI1 lock interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PLLSAI1 lock interrupt enabled"]
    Enabled = 1,
}
impl From<PLLSAI1RDYIE> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_R = crate::BitReader<PLLSAI1RDYIE>;
impl PLLSAI1RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1RDYIE {
        match self.bits {
            false => PLLSAI1RDYIE::Disabled,
            true => PLLSAI1RDYIE::Enabled,
        }
    }
    #[doc = "PLLSAI1 lock interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1RDYIE::Disabled
    }
    #[doc = "PLLSAI1 lock interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1RDYIE::Enabled
    }
}
#[doc = "Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1RDYIE>;
impl<'a, REG> PLLSAI1RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLSAI1 lock interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1RDYIE::Disabled)
    }
    #[doc = "PLLSAI1 lock interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1RDYIE::Enabled)
    }
}
#[doc = "PLLSAI2 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYIE {
    #[doc = "0: PLLSAI2 lock interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PLLSAI2 lock interrupt enabled"]
    Enabled = 1,
}
impl From<PLLSAI2RDYIE> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2RDYIE` reader - PLLSAI2 ready interrupt enable"]
pub type PLLSAI2RDYIE_R = crate::BitReader<PLLSAI2RDYIE>;
impl PLLSAI2RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2RDYIE {
        match self.bits {
            false => PLLSAI2RDYIE::Disabled,
            true => PLLSAI2RDYIE::Enabled,
        }
    }
    #[doc = "PLLSAI2 lock interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2RDYIE::Disabled
    }
    #[doc = "PLLSAI2 lock interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2RDYIE::Enabled
    }
}
#[doc = "Field `PLLSAI2RDYIE` writer - PLLSAI2 ready interrupt enable"]
pub type PLLSAI2RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2RDYIE>;
impl<'a, REG> PLLSAI2RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLSAI2 lock interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2RDYIE::Disabled)
    }
    #[doc = "PLLSAI2 lock interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2RDYIE::Enabled)
    }
}
#[doc = "LSE clock security system interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE {
    #[doc = "0: Clock security interrupt caused by LSE clock failure disabled"]
    Disabled = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure enabled"]
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
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSIE::Disabled
    }
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
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
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Disabled)
    }
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Enabled)
    }
}
#[doc = "HSI48 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYIE {
    #[doc = "0: HSI48 ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HSI48 ready interrupt enabled"]
    Enabled = 1,
}
impl From<HSI48RDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_R = crate::BitReader<HSI48RDYIE>;
impl HSI48RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYIE {
        match self.bits {
            false => HSI48RDYIE::Disabled,
            true => HSI48RDYIE::Enabled,
        }
    }
    #[doc = "HSI48 ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48RDYIE::Disabled
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48RDYIE::Enabled
    }
}
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSI48RDYIE>;
impl<'a, REG> HSI48RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYIE::Disabled)
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> PLLSAI2RDYIE_R {
        PLLSAI2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<CIERrs> {
        MSIRDYIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIERrs> {
        PLLRDYIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<CIERrs> {
        PLLSAI1RDYIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2rdyie(&mut self) -> PLLSAI2RDYIE_W<CIERrs> {
        PLLSAI2RDYIE_W::new(self, 7)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<CIERrs> {
        HSI48RDYIE_W::new(self, 10)
    }
}
#[doc = "Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIERrs {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIERrs {
    const RESET_VALUE: u32 = 0;
}
