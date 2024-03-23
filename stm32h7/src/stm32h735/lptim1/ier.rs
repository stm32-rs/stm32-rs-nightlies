#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Compare match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMIE {
    #[doc = "0: CMPM interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CMPM interrupt enabled"]
    Enabled = 1,
}
impl From<CMPMIE> for bool {
    #[inline(always)]
    fn from(variant: CMPMIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMIE` reader - Compare match Interrupt Enable"]
pub type CMPMIE_R = crate::BitReader<CMPMIE>;
impl CMPMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPMIE {
        match self.bits {
            false => CMPMIE::Disabled,
            true => CMPMIE::Enabled,
        }
    }
    #[doc = "CMPM interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPMIE::Disabled
    }
    #[doc = "CMPM interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPMIE::Enabled
    }
}
#[doc = "Field `CMPMIE` writer - Compare match Interrupt Enable"]
pub type CMPMIE_W<'a, REG> = crate::BitWriter<'a, REG, CMPMIE>;
impl<'a, REG> CMPMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPM interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMIE::Disabled)
    }
    #[doc = "CMPM interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMIE::Enabled)
    }
}
#[doc = "Autoreload match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMIE {
    #[doc = "0: ARRM interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ARRM interrupt enabled"]
    Enabled = 1,
}
impl From<ARRMIE> for bool {
    #[inline(always)]
    fn from(variant: ARRMIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader<ARRMIE>;
impl ARRMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARRMIE {
        match self.bits {
            false => ARRMIE::Disabled,
            true => ARRMIE::Enabled,
        }
    }
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARRMIE::Disabled
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARRMIE::Enabled
    }
}
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG, ARRMIE>;
impl<'a, REG> ARRMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMIE::Disabled)
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMIE::Enabled)
    }
}
#[doc = "External trigger valid edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGIE {
    #[doc = "0: EXTTRIG interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EXTTRIG interrupt enabled"]
    Enabled = 1,
}
impl From<EXTTRIGIE> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader<EXTTRIGIE>;
impl EXTTRIGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTTRIGIE {
        match self.bits {
            false => EXTTRIGIE::Disabled,
            true => EXTTRIGIE::Enabled,
        }
    }
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIGIE::Disabled
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIGIE::Enabled
    }
}
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG, EXTTRIGIE>;
impl<'a, REG> EXTTRIGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGIE::Disabled)
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGIE::Enabled)
    }
}
#[doc = "Compare register update OK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKIE {
    #[doc = "0: CMPOK interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CMPOK interrupt enabled"]
    Enabled = 1,
}
impl From<CMPOKIE> for bool {
    #[inline(always)]
    fn from(variant: CMPOKIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOKIE` reader - Compare register update OK Interrupt Enable"]
pub type CMPOKIE_R = crate::BitReader<CMPOKIE>;
impl CMPOKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPOKIE {
        match self.bits {
            false => CMPOKIE::Disabled,
            true => CMPOKIE::Enabled,
        }
    }
    #[doc = "CMPOK interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPOKIE::Disabled
    }
    #[doc = "CMPOK interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPOKIE::Enabled
    }
}
#[doc = "Field `CMPOKIE` writer - Compare register update OK Interrupt Enable"]
pub type CMPOKIE_W<'a, REG> = crate::BitWriter<'a, REG, CMPOKIE>;
impl<'a, REG> CMPOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOKIE::Disabled)
    }
    #[doc = "CMPOK interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOKIE::Enabled)
    }
}
#[doc = "Autoreload register update OK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKIE {
    #[doc = "0: ARROK interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ARROK interrupt enabled"]
    Enabled = 1,
}
impl From<ARROKIE> for bool {
    #[inline(always)]
    fn from(variant: ARROKIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader<ARROKIE>;
impl ARROKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARROKIE {
        match self.bits {
            false => ARROKIE::Disabled,
            true => ARROKIE::Enabled,
        }
    }
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARROKIE::Disabled
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARROKIE::Enabled
    }
}
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG, ARROKIE>;
impl<'a, REG> ARROKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKIE::Disabled)
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKIE::Enabled)
    }
}
#[doc = "Direction change to UP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPIE {
    #[doc = "0: UP interrupt disabled"]
    Disabled = 0,
    #[doc = "1: UP interrupt enabled"]
    Enabled = 1,
}
impl From<UPIE> for bool {
    #[inline(always)]
    fn from(variant: UPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable"]
pub type UPIE_R = crate::BitReader<UPIE>;
impl UPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPIE {
        match self.bits {
            false => UPIE::Disabled,
            true => UPIE::Enabled,
        }
    }
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPIE::Disabled
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPIE::Enabled
    }
}
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable"]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG, UPIE>;
impl<'a, REG> UPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE::Disabled)
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE::Enabled)
    }
}
#[doc = "Direction change to down Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNIE {
    #[doc = "0: DOWN interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DOWN interrupt enabled"]
    Enabled = 1,
}
impl From<DOWNIE> for bool {
    #[inline(always)]
    fn from(variant: DOWNIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable"]
pub type DOWNIE_R = crate::BitReader<DOWNIE>;
impl DOWNIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOWNIE {
        match self.bits {
            false => DOWNIE::Disabled,
            true => DOWNIE::Enabled,
        }
    }
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWNIE::Disabled
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWNIE::Enabled
    }
}
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable"]
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG, DOWNIE>;
impl<'a, REG> DOWNIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNIE::Disabled)
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmie(&mut self) -> CMPMIE_W<IERrs> {
        CMPMIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<IERrs> {
        ARRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<IERrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<IERrs> {
        CMPOKIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<IERrs> {
        ARROKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<IERrs> {
        UPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<IERrs> {
        DOWNIE_W::new(self, 6)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
