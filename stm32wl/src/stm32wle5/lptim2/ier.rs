///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**Compare match Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMIE {
    ///0: CMPM interrupt disabled
    Disabled = 0,
    ///1: CMPM interrupt enabled
    Enabled = 1,
}
impl From<CMPMIE> for bool {
    #[inline(always)]
    fn from(variant: CMPMIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPMIE` reader - Compare match Interrupt Enable
pub type CMPMIE_R = crate::BitReader<CMPMIE>;
impl CMPMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPMIE {
        match self.bits {
            false => CMPMIE::Disabled,
            true => CMPMIE::Enabled,
        }
    }
    ///CMPM interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPMIE::Disabled
    }
    ///CMPM interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPMIE::Enabled
    }
}
///Field `CMPMIE` writer - Compare match Interrupt Enable
pub type CMPMIE_W<'a, REG> = crate::BitWriter<'a, REG, CMPMIE>;
impl<'a, REG> CMPMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CMPM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMIE::Disabled)
    }
    ///CMPM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMIE::Enabled)
    }
}
/**Autoreload match Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMIE {
    ///0: ARRM interrupt disabled
    Disabled = 0,
    ///1: ARRM interrupt enabled
    Enabled = 1,
}
impl From<ARRMIE> for bool {
    #[inline(always)]
    fn from(variant: ARRMIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ARRMIE` reader - Autoreload match Interrupt Enable
pub type ARRMIE_R = crate::BitReader<ARRMIE>;
impl ARRMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARRMIE {
        match self.bits {
            false => ARRMIE::Disabled,
            true => ARRMIE::Enabled,
        }
    }
    ///ARRM interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARRMIE::Disabled
    }
    ///ARRM interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARRMIE::Enabled
    }
}
///Field `ARRMIE` writer - Autoreload match Interrupt Enable
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG, ARRMIE>;
impl<'a, REG> ARRMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ARRM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMIE::Disabled)
    }
    ///ARRM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMIE::Enabled)
    }
}
/**External trigger valid edge Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGIE {
    ///0: EXTTRIG interrupt disabled
    Disabled = 0,
    ///1: EXTTRIG interrupt enabled
    Enabled = 1,
}
impl From<EXTTRIGIE> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_R = crate::BitReader<EXTTRIGIE>;
impl EXTTRIGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTTRIGIE {
        match self.bits {
            false => EXTTRIGIE::Disabled,
            true => EXTTRIGIE::Enabled,
        }
    }
    ///EXTTRIG interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIGIE::Disabled
    }
    ///EXTTRIG interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIGIE::Enabled
    }
}
///Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG, EXTTRIGIE>;
impl<'a, REG> EXTTRIGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EXTTRIG interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGIE::Disabled)
    }
    ///EXTTRIG interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGIE::Enabled)
    }
}
/**Compare register update OK Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKIE {
    ///0: CMPOK interrupt disabled
    Disabled = 0,
    ///1: CMPOK interrupt enabled
    Enabled = 1,
}
impl From<CMPOKIE> for bool {
    #[inline(always)]
    fn from(variant: CMPOKIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOKIE` reader - Compare register update OK Interrupt Enable
pub type CMPOKIE_R = crate::BitReader<CMPOKIE>;
impl CMPOKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPOKIE {
        match self.bits {
            false => CMPOKIE::Disabled,
            true => CMPOKIE::Enabled,
        }
    }
    ///CMPOK interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPOKIE::Disabled
    }
    ///CMPOK interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPOKIE::Enabled
    }
}
///Field `CMPOKIE` writer - Compare register update OK Interrupt Enable
pub type CMPOKIE_W<'a, REG> = crate::BitWriter<'a, REG, CMPOKIE>;
impl<'a, REG> CMPOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CMPOK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOKIE::Disabled)
    }
    ///CMPOK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOKIE::Enabled)
    }
}
/**Autoreload register update OK Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKIE {
    ///0: ARROK interrupt disabled
    Disabled = 0,
    ///1: ARROK interrupt enabled
    Enabled = 1,
}
impl From<ARROKIE> for bool {
    #[inline(always)]
    fn from(variant: ARROKIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable
pub type ARROKIE_R = crate::BitReader<ARROKIE>;
impl ARROKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARROKIE {
        match self.bits {
            false => ARROKIE::Disabled,
            true => ARROKIE::Enabled,
        }
    }
    ///ARROK interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARROKIE::Disabled
    }
    ///ARROK interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARROKIE::Enabled
    }
}
///Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG, ARROKIE>;
impl<'a, REG> ARROKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ARROK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKIE::Disabled)
    }
    ///ARROK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKIE::Enabled)
    }
}
/**Direction change to UP Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPIE {
    ///0: UP interrupt disabled
    Disabled = 0,
    ///1: UP interrupt enabled
    Enabled = 1,
}
impl From<UPIE> for bool {
    #[inline(always)]
    fn from(variant: UPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UPIE` reader - Direction change to UP Interrupt Enable
pub type UPIE_R = crate::BitReader<UPIE>;
impl UPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPIE {
        match self.bits {
            false => UPIE::Disabled,
            true => UPIE::Enabled,
        }
    }
    ///UP interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPIE::Disabled
    }
    ///UP interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPIE::Enabled
    }
}
///Field `UPIE` writer - Direction change to UP Interrupt Enable
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG, UPIE>;
impl<'a, REG> UPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE::Disabled)
    }
    ///UP interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE::Enabled)
    }
}
/**Direction change to down Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNIE {
    ///0: DOWN interrupt disabled
    Disabled = 0,
    ///1: DOWN interrupt enabled
    Enabled = 1,
}
impl From<DOWNIE> for bool {
    #[inline(always)]
    fn from(variant: DOWNIE) -> Self {
        variant as u8 != 0
    }
}
///Field `DOWNIE` reader - Direction change to down Interrupt Enable
pub type DOWNIE_R = crate::BitReader<DOWNIE>;
impl DOWNIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOWNIE {
        match self.bits {
            false => DOWNIE::Disabled,
            true => DOWNIE::Enabled,
        }
    }
    ///DOWN interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWNIE::Disabled
    }
    ///DOWN interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWNIE::Enabled
    }
}
///Field `DOWNIE` writer - Direction change to down Interrupt Enable
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG, DOWNIE>;
impl<'a, REG> DOWNIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DOWN interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNIE::Disabled)
    }
    ///DOWN interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNIE::Enabled)
    }
}
/**Update event interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UEIE {
    ///0: Update event interrupt disabled
    Disabled = 0,
    ///1: Update event interrupt enabled
    Enabled = 1,
}
impl From<UEIE> for bool {
    #[inline(always)]
    fn from(variant: UEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UEIE` reader - Update event interrupt enable
pub type UEIE_R = crate::BitReader<UEIE>;
impl UEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UEIE {
        match self.bits {
            false => UEIE::Disabled,
            true => UEIE::Enabled,
        }
    }
    ///Update event interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UEIE::Disabled
    }
    ///Update event interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UEIE::Enabled
    }
}
///Field `UEIE` writer - Update event interrupt enable
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG, UEIE>;
impl<'a, REG> UEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update event interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UEIE::Disabled)
    }
    ///Update event interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UEIE::Enabled)
    }
}
/**Repetition register update OK interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPOKIE {
    ///0: Repetition register update OK interrupt disabled
    Disabled = 0,
    ///1: Repetition register update OK interrupt enabled
    Enabled = 1,
}
impl From<REPOKIE> for bool {
    #[inline(always)]
    fn from(variant: REPOKIE) -> Self {
        variant as u8 != 0
    }
}
///Field `REPOKIE` reader - Repetition register update OK interrupt Enable
pub type REPOKIE_R = crate::BitReader<REPOKIE>;
impl REPOKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REPOKIE {
        match self.bits {
            false => REPOKIE::Disabled,
            true => REPOKIE::Enabled,
        }
    }
    ///Repetition register update OK interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPOKIE::Disabled
    }
    ///Repetition register update OK interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPOKIE::Enabled
    }
}
///Field `REPOKIE` writer - Repetition register update OK interrupt Enable
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG, REPOKIE>;
impl<'a, REG> REPOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Repetition register update OK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPOKIE::Disabled)
    }
    ///Repetition register update OK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPOKIE::Enabled)
    }
}
impl R {
    ///Bit 0 - Compare match Interrupt Enable
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register update OK Interrupt Enable
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("repokie", &self.repokie())
            .field("ueie", &self.ueie())
            .field("downie", &self.downie())
            .field("upie", &self.upie())
            .field("arrokie", &self.arrokie())
            .field("cmpokie", &self.cmpokie())
            .field("exttrigie", &self.exttrigie())
            .field("arrmie", &self.arrmie())
            .field("cmpmie", &self.cmpmie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare match Interrupt Enable
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CMPMIE_W<'_, IERrs> {
        CMPMIE_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<'_, IERrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<'_, IERrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 3 - Compare register update OK Interrupt Enable
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<'_, IERrs> {
        CMPOKIE_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<'_, IERrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<'_, IERrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<'_, IERrs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<'_, IERrs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<'_, IERrs> {
        REPOKIE_W::new(self, 8)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#LPTIM2:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
