#[doc = "Register `CNSLCKR` reader"]
pub type R = crate::R<CNSLCKRrs>;
#[doc = "Register `CNSLCKR` writer"]
pub type W = crate::W<CNSLCKRrs>;
#[doc = "VTOR_NS register lock This bit is set by software and cleared only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKNSVTOR {
    #[doc = "0: VTOR_NS register write enabled"]
    Unlocked = 0,
    #[doc = "1: VTOR_NS register write disabled"]
    Locked = 1,
}
impl From<LOCKNSVTOR> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSVTOR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKNSVTOR` reader - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
pub type LOCKNSVTOR_R = crate::BitReader<LOCKNSVTOR>;
impl LOCKNSVTOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKNSVTOR {
        match self.bits {
            false => LOCKNSVTOR::Unlocked,
            true => LOCKNSVTOR::Locked,
        }
    }
    #[doc = "VTOR_NS register write enabled"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKNSVTOR::Unlocked
    }
    #[doc = "VTOR_NS register write disabled"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKNSVTOR::Locked
    }
}
#[doc = "Field `LOCKNSVTOR` writer - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
pub type LOCKNSVTOR_W<'a, REG> = crate::BitWriter<'a, REG, LOCKNSVTOR>;
impl<'a, REG> LOCKNSVTOR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VTOR_NS register write enabled"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSVTOR::Unlocked)
    }
    #[doc = "VTOR_NS register write disabled"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSVTOR::Locked)
    }
}
#[doc = "MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKNSMPU {
    #[doc = "0: MPU registers write enabled"]
    Unlocked = 0,
    #[doc = "1: MPU registers write disabled"]
    Locked = 1,
}
impl From<LOCKNSMPU> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSMPU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKNSMPU` reader - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
pub type LOCKNSMPU_R = crate::BitReader<LOCKNSMPU>;
impl LOCKNSMPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKNSMPU {
        match self.bits {
            false => LOCKNSMPU::Unlocked,
            true => LOCKNSMPU::Locked,
        }
    }
    #[doc = "MPU registers write enabled"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKNSMPU::Unlocked
    }
    #[doc = "MPU registers write disabled"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKNSMPU::Locked
    }
}
#[doc = "Field `LOCKNSMPU` writer - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
pub type LOCKNSMPU_W<'a, REG> = crate::BitWriter<'a, REG, LOCKNSMPU>;
impl<'a, REG> LOCKNSMPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU registers write enabled"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSMPU::Unlocked)
    }
    #[doc = "MPU registers write disabled"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSMPU::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<CNSLCKRrs> {
        LOCKNSVTOR_W::new(self, 0)
    }
    #[doc = "Bit 1 - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<CNSLCKRrs> {
        LOCKNSMPU_W::new(self, 1)
    }
}
#[doc = "SBS CPU lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnslckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnslckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNSLCKRrs;
impl crate::RegisterSpec for CNSLCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnslckr::R`](R) reader structure"]
impl crate::Readable for CNSLCKRrs {}
#[doc = "`write(|w| ..)` method takes [`cnslckr::W`](W) writer structure"]
impl crate::Writable for CNSLCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNSLCKR to value 0"]
impl crate::Resettable for CNSLCKRrs {
    const RESET_VALUE: u32 = 0;
}
