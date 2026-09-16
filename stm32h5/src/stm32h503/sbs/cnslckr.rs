///Register `CNSLCKR` reader
pub type R = crate::R<CNSLCKRrs>;
///Register `CNSLCKR` writer
pub type W = crate::W<CNSLCKRrs>;
/**VTOR_NS register lock This bit is set by software and cleared only by a system reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKNSVTOR {
    ///0: VTOR_NS register write enabled
    Unlocked = 0,
    ///1: VTOR_NS register write disabled
    Locked = 1,
}
impl From<LOCKNSVTOR> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSVTOR) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCKNSVTOR` reader - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
pub type LOCKNSVTOR_R = crate::BitReader<LOCKNSVTOR>;
impl LOCKNSVTOR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCKNSVTOR {
        match self.bits {
            false => LOCKNSVTOR::Unlocked,
            true => LOCKNSVTOR::Locked,
        }
    }
    ///VTOR_NS register write enabled
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKNSVTOR::Unlocked
    }
    ///VTOR_NS register write disabled
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKNSVTOR::Locked
    }
}
///Field `LOCKNSVTOR` writer - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
pub type LOCKNSVTOR_W<'a, REG> = crate::BitWriter<'a, REG, LOCKNSVTOR>;
impl<'a, REG> LOCKNSVTOR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VTOR_NS register write enabled
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSVTOR::Unlocked)
    }
    ///VTOR_NS register write disabled
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSVTOR::Locked)
    }
}
/**MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKNSMPU {
    ///0: MPU registers write enabled
    Unlocked = 0,
    ///1: MPU registers write disabled
    Locked = 1,
}
impl From<LOCKNSMPU> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSMPU) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCKNSMPU` reader - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
pub type LOCKNSMPU_R = crate::BitReader<LOCKNSMPU>;
impl LOCKNSMPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCKNSMPU {
        match self.bits {
            false => LOCKNSMPU::Unlocked,
            true => LOCKNSMPU::Locked,
        }
    }
    ///MPU registers write enabled
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKNSMPU::Unlocked
    }
    ///MPU registers write disabled
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKNSMPU::Locked
    }
}
///Field `LOCKNSMPU` writer - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
pub type LOCKNSMPU_W<'a, REG> = crate::BitWriter<'a, REG, LOCKNSMPU>;
impl<'a, REG> LOCKNSMPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MPU registers write enabled
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSMPU::Unlocked)
    }
    ///MPU registers write disabled
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSMPU::Locked)
    }
}
impl R {
    ///Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNSLCKR")
            .field("locknsvtor", &self.locknsvtor())
            .field("locknsmpu", &self.locknsmpu())
            .finish()
    }
}
impl W {
    ///Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<'_, CNSLCKRrs> {
        LOCKNSVTOR_W::new(self, 0)
    }
    ///Bit 1 - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<'_, CNSLCKRrs> {
        LOCKNSMPU_W::new(self, 1)
    }
}
/**SBS CPU lock register

You can [`read`](crate::Reg::read) this register and get [`cnslckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CNSLCKR)*/
pub struct CNSLCKRrs;
impl crate::RegisterSpec for CNSLCKRrs {
    type Ux = u32;
}
///`read()` method returns [`cnslckr::R`](R) reader structure
impl crate::Readable for CNSLCKRrs {}
///`write(|w| ..)` method takes [`cnslckr::W`](W) writer structure
impl crate::Writable for CNSLCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNSLCKR to value 0
impl crate::Resettable for CNSLCKRrs {}
