///Register `OPAMP1_TCMR` reader
pub type R = crate::R<OPAMP1_TCMRrs>;
///Register `OPAMP1_TCMR` writer
pub type W = crate::W<OPAMP1_TCMRrs>;
///Field `VMS_SEL` reader - VMS_SEL
pub type VMS_SEL_R = crate::BitReader;
///Field `VMS_SEL` writer - VMS_SEL
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///VPS_SEL
pub use super::opamp1_csr::VP_SEL;
///Field `VPS_SEL` reader - VPS_SEL
pub use super::opamp1_csr::VP_SEL_R as VPS_SEL_R;
///Field `VPS_SEL` writer - VPS_SEL
pub use super::opamp1_csr::VP_SEL_W as VPS_SEL_W;
/**T1CM_EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1CM_EN {
    ///0: Automatic input switch triggered by TIM1 disabled
    Disabled = 0,
    ///1: Automatic input switch triggered by TIM1 enabled
    Enabled = 1,
}
impl From<T1CM_EN> for bool {
    #[inline(always)]
    fn from(variant: T1CM_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `T1CM_EN` reader - T1CM_EN
pub type T1CM_EN_R = crate::BitReader<T1CM_EN>;
impl T1CM_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> T1CM_EN {
        match self.bits {
            false => T1CM_EN::Disabled,
            true => T1CM_EN::Enabled,
        }
    }
    ///Automatic input switch triggered by TIM1 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T1CM_EN::Disabled
    }
    ///Automatic input switch triggered by TIM1 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T1CM_EN::Enabled
    }
}
///Field `T1CM_EN` writer - T1CM_EN
pub type T1CM_EN_W<'a, REG> = crate::BitWriter<'a, REG, T1CM_EN>;
impl<'a, REG> T1CM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic input switch triggered by TIM1 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(T1CM_EN::Disabled)
    }
    ///Automatic input switch triggered by TIM1 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(T1CM_EN::Enabled)
    }
}
/**T8CM_EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T8CM_EN {
    ///0: Automatic input switch triggered by TIM8 disabled
    Disabled = 0,
    ///1: Automatic input switch triggered by TIM8 enabled
    Enabled = 1,
}
impl From<T8CM_EN> for bool {
    #[inline(always)]
    fn from(variant: T8CM_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `T8CM_EN` reader - T8CM_EN
pub type T8CM_EN_R = crate::BitReader<T8CM_EN>;
impl T8CM_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> T8CM_EN {
        match self.bits {
            false => T8CM_EN::Disabled,
            true => T8CM_EN::Enabled,
        }
    }
    ///Automatic input switch triggered by TIM8 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T8CM_EN::Disabled
    }
    ///Automatic input switch triggered by TIM8 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T8CM_EN::Enabled
    }
}
///Field `T8CM_EN` writer - T8CM_EN
pub type T8CM_EN_W<'a, REG> = crate::BitWriter<'a, REG, T8CM_EN>;
impl<'a, REG> T8CM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic input switch triggered by TIM8 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(T8CM_EN::Disabled)
    }
    ///Automatic input switch triggered by TIM8 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(T8CM_EN::Enabled)
    }
}
/**T20CM_EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T20CM_EN {
    ///0: Automatic input switch triggered by TIM20 disabled
    Disabled = 0,
    ///1: Automatic input switch triggered by TIM20 enabled
    Enabled = 1,
}
impl From<T20CM_EN> for bool {
    #[inline(always)]
    fn from(variant: T20CM_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `T20CM_EN` reader - T20CM_EN
pub type T20CM_EN_R = crate::BitReader<T20CM_EN>;
impl T20CM_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> T20CM_EN {
        match self.bits {
            false => T20CM_EN::Disabled,
            true => T20CM_EN::Enabled,
        }
    }
    ///Automatic input switch triggered by TIM20 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T20CM_EN::Disabled
    }
    ///Automatic input switch triggered by TIM20 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T20CM_EN::Enabled
    }
}
///Field `T20CM_EN` writer - T20CM_EN
pub type T20CM_EN_W<'a, REG> = crate::BitWriter<'a, REG, T20CM_EN>;
impl<'a, REG> T20CM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic input switch triggered by TIM20 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(T20CM_EN::Disabled)
    }
    ///Automatic input switch triggered by TIM20 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(T20CM_EN::Enabled)
    }
}
/**LOCK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    ///0: TCMR is read-write
    ReadWrite = 0,
    ///1: TCMR is read-only, can only be cleared by system reset
    ReadOnly = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::ReadWrite,
            true => LOCK::ReadOnly,
        }
    }
    ///TCMR is read-write
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK::ReadWrite
    }
    ///TCMR is read-only, can only be cleared by system reset
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK::ReadOnly
    }
}
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TCMR is read-write
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::ReadWrite)
    }
    ///TCMR is read-only, can only be cleared by system reset
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::ReadOnly)
    }
}
impl R {
    ///Bit 0 - VMS_SEL
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - VPS_SEL
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - T1CM_EN
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - T8CM_EN
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - T20CM_EN
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP1_TCMR")
            .field("vms_sel", &self.vms_sel())
            .field("vps_sel", &self.vps_sel())
            .field("t1cm_en", &self.t1cm_en())
            .field("t8cm_en", &self.t8cm_en())
            .field("t20cm_en", &self.t20cm_en())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - VMS_SEL
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<'_, OPAMP1_TCMRrs> {
        VMS_SEL_W::new(self, 0)
    }
    ///Bits 1:2 - VPS_SEL
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<'_, OPAMP1_TCMRrs> {
        VPS_SEL_W::new(self, 1)
    }
    ///Bit 3 - T1CM_EN
    #[inline(always)]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W<'_, OPAMP1_TCMRrs> {
        T1CM_EN_W::new(self, 3)
    }
    ///Bit 4 - T8CM_EN
    #[inline(always)]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W<'_, OPAMP1_TCMRrs> {
        T8CM_EN_W::new(self, 4)
    }
    ///Bit 5 - T20CM_EN
    #[inline(always)]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W<'_, OPAMP1_TCMRrs> {
        T20CM_EN_W::new(self, 5)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, OPAMP1_TCMRrs> {
        LOCK_W::new(self, 31)
    }
}
/**OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_tcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_tcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#OPAMP:OPAMP1_TCMR)*/
pub struct OPAMP1_TCMRrs;
impl crate::RegisterSpec for OPAMP1_TCMRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp1_tcmr::R`](R) reader structure
impl crate::Readable for OPAMP1_TCMRrs {}
///`write(|w| ..)` method takes [`opamp1_tcmr::W`](W) writer structure
impl crate::Writable for OPAMP1_TCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP1_TCMR to value 0
impl crate::Resettable for OPAMP1_TCMRrs {}
