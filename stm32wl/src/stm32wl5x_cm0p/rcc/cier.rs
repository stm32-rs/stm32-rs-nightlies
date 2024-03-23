#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIERrs>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIERrs>;
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
#[doc = "LSE ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSERDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::Enabled)
    }
}
#[doc = "MSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIRDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYIE::Enabled)
    }
}
#[doc = "HSI16 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<HSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI16 ready interrupt enable"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIRDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIRDYIE::Enabled
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI16 ready interrupt enable"]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYIE>;
impl<'a, REG> HSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::Enabled)
    }
}
#[doc = "HSE32 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<HSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYIE` reader - HSE32 ready interrupt enable"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSERDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSERDYIE::Enabled
    }
}
#[doc = "Field `HSERDYIE` writer - HSE32 ready interrupt enable"]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYIE>;
impl<'a, REG> HSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::Enabled)
    }
}
#[doc = "PLL ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLRDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYIE::Enabled)
    }
}
#[doc = "LSE clock security system interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSIE::Disabled
    }
    #[doc = "Interrupt enabled"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSIE::Enabled)
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
    #[doc = "Bit 3 - HSI16 ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE32 ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 3 - HSI16 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE32 ready interrupt enable"]
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
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<CIERrs> {
        LSECSSIE_W::new(self, 9)
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
