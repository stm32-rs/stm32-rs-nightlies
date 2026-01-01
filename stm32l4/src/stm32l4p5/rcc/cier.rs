///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
/**LSI ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: LSI ready interrupt disabled
    Disabled = 0,
    ///1: LSI ready interrupt enabled
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
    ///LSI ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    ///LSI ready interrupt enabled
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
    ///LSI ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    ///LSI ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
/**LSE ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE {
    ///0: LSE ready interrupt disabled
    Disabled = 0,
    ///1: LSE ready interrupt enabled
    Enabled = 1,
}
impl From<LSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub type LSERDYIE_R = crate::BitReader<LSERDYIE>;
impl LSERDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYIE {
        match self.bits {
            false => LSERDYIE::Disabled,
            true => LSERDYIE::Enabled,
        }
    }
    ///LSE ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSERDYIE::Disabled
    }
    ///LSE ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSERDYIE::Enabled
    }
}
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYIE>;
impl<'a, REG> LSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::Disabled)
    }
    ///LSE ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::Enabled)
    }
}
/**MSI ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYIE {
    ///0: MSI ready interrupt disabled
    Disabled = 0,
    ///1: MSI ready interrupt enabled
    Enabled = 1,
}
impl From<MSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub type MSIRDYIE_R = crate::BitReader<MSIRDYIE>;
impl MSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYIE {
        match self.bits {
            false => MSIRDYIE::Disabled,
            true => MSIRDYIE::Enabled,
        }
    }
    ///MSI ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIRDYIE::Disabled
    }
    ///MSI ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIRDYIE::Enabled
    }
}
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, MSIRDYIE>;
impl<'a, REG> MSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYIE::Disabled)
    }
    ///MSI ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYIE::Enabled)
    }
}
/**HSI ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE {
    ///0: HSI16 ready interrupt disabled
    Disabled = 0,
    ///1: HSI16 ready interrupt enabled
    Enabled = 1,
}
impl From<HSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYIE` reader - HSI ready interrupt enable
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE>;
impl HSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYIE {
        match self.bits {
            false => HSIRDYIE::Disabled,
            true => HSIRDYIE::Enabled,
        }
    }
    ///HSI16 ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIRDYIE::Disabled
    }
    ///HSI16 ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIRDYIE::Enabled
    }
}
///Field `HSIRDYIE` writer - HSI ready interrupt enable
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYIE>;
impl<'a, REG> HSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::Disabled)
    }
    ///HSI16 ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::Enabled)
    }
}
/**HSE ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE {
    ///0: HSE ready interrupt disabled
    Disabled = 0,
    ///1: HSE ready interrupt enabled
    Enabled = 1,
}
impl From<HSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYIE` reader - HSE ready interrupt enable
pub type HSERDYIE_R = crate::BitReader<HSERDYIE>;
impl HSERDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYIE {
        match self.bits {
            false => HSERDYIE::Disabled,
            true => HSERDYIE::Enabled,
        }
    }
    ///HSE ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSERDYIE::Disabled
    }
    ///HSE ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSERDYIE::Enabled
    }
}
///Field `HSERDYIE` writer - HSE ready interrupt enable
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYIE>;
impl<'a, REG> HSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::Disabled)
    }
    ///HSE ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::Enabled)
    }
}
/**PLL ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYIE {
    ///0: PLL lock interrupt disabled
    Disabled = 0,
    ///1: PLL lock interrupt enabled
    Enabled = 1,
}
impl From<PLLRDYIE> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDYIE` reader - PLL ready interrupt enable
pub type PLLRDYIE_R = crate::BitReader<PLLRDYIE>;
impl PLLRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDYIE {
        match self.bits {
            false => PLLRDYIE::Disabled,
            true => PLLRDYIE::Enabled,
        }
    }
    ///PLL lock interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLRDYIE::Disabled
    }
    ///PLL lock interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLRDYIE::Enabled
    }
}
///Field `PLLRDYIE` writer - PLL ready interrupt enable
pub type PLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLRDYIE>;
impl<'a, REG> PLLRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL lock interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYIE::Disabled)
    }
    ///PLL lock interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYIE::Enabled)
    }
}
/**PLLSAI1 ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYIE {
    ///0: PLLSAI1 lock interrupt disabled
    Disabled = 0,
    ///1: PLLSAI1 lock interrupt enabled
    Enabled = 1,
}
impl From<PLLSAI1RDYIE> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable
pub type PLLSAI1RDYIE_R = crate::BitReader<PLLSAI1RDYIE>;
impl PLLSAI1RDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1RDYIE {
        match self.bits {
            false => PLLSAI1RDYIE::Disabled,
            true => PLLSAI1RDYIE::Enabled,
        }
    }
    ///PLLSAI1 lock interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1RDYIE::Disabled
    }
    ///PLLSAI1 lock interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1RDYIE::Enabled
    }
}
///Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable
pub type PLLSAI1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1RDYIE>;
impl<'a, REG> PLLSAI1RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI1 lock interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1RDYIE::Disabled)
    }
    ///PLLSAI1 lock interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1RDYIE::Enabled)
    }
}
/**PLLSAI2 ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYIE {
    ///0: PLLSAI2 lock interrupt disabled
    Disabled = 0,
    ///1: PLLSAI2 lock interrupt enabled
    Enabled = 1,
}
impl From<PLLSAI2RDYIE> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI2RDYIE` reader - PLLSAI2 ready interrupt enable
pub type PLLSAI2RDYIE_R = crate::BitReader<PLLSAI2RDYIE>;
impl PLLSAI2RDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2RDYIE {
        match self.bits {
            false => PLLSAI2RDYIE::Disabled,
            true => PLLSAI2RDYIE::Enabled,
        }
    }
    ///PLLSAI2 lock interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2RDYIE::Disabled
    }
    ///PLLSAI2 lock interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2RDYIE::Enabled
    }
}
///Field `PLLSAI2RDYIE` writer - PLLSAI2 ready interrupt enable
pub type PLLSAI2RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2RDYIE>;
impl<'a, REG> PLLSAI2RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI2 lock interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2RDYIE::Disabled)
    }
    ///PLLSAI2 lock interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2RDYIE::Enabled)
    }
}
/**LSE clock security system interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE {
    ///0: Clock security interrupt caused by LSE clock failure disabled
    Disabled = 0,
    ///1: Clock security interrupt caused by LSE clock failure enabled
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
    ///Clock security interrupt caused by LSE clock failure disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSIE::Disabled
    }
    ///Clock security interrupt caused by LSE clock failure enabled
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
    ///Clock security interrupt caused by LSE clock failure disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Disabled)
    }
    ///Clock security interrupt caused by LSE clock failure enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Enabled)
    }
}
/**HSI48 ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYIE {
    ///0: HSI48 ready interrupt disabled
    Disabled = 0,
    ///1: HSI48 ready interrupt enabled
    Enabled = 1,
}
impl From<HSI48RDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable
pub type HSI48RDYIE_R = crate::BitReader<HSI48RDYIE>;
impl HSI48RDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYIE {
        match self.bits {
            false => HSI48RDYIE::Disabled,
            true => HSI48RDYIE::Enabled,
        }
    }
    ///HSI48 ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48RDYIE::Disabled
    }
    ///HSI48 ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48RDYIE::Enabled
    }
}
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSI48RDYIE>;
impl<'a, REG> HSI48RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI48 ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYIE::Disabled)
    }
    ///HSI48 ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYIE::Enabled)
    }
}
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
    ///Bit 5 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLLSAI2 ready interrupt enable
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> PLLSAI2RDYIE_R {
        PLLSAI2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
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
            .field("pllrdyie", &self.pllrdyie())
            .field("pllsai1rdyie", &self.pllsai1rdyie())
            .field("pllsai2rdyie", &self.pllsai2rdyie())
            .field("lsecssie", &self.lsecssie())
            .field("hsi48rdyie", &self.hsi48rdyie())
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
    ///Bit 5 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIERrs> {
        PLLRDYIE_W::new(self, 5)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<'_, CIERrs> {
        PLLSAI1RDYIE_W::new(self, 6)
    }
    ///Bit 7 - PLLSAI2 ready interrupt enable
    #[inline(always)]
    pub fn pllsai2rdyie(&mut self) -> PLLSAI2RDYIE_W<'_, CIERrs> {
        PLLSAI2RDYIE_W::new(self, 7)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<'_, CIERrs> {
        HSI48RDYIE_W::new(self, 10)
    }
}
/**Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:CIER)*/
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
