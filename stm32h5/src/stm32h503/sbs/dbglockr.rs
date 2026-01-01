///Register `DBGLOCKR` reader
pub type R = crate::R<DBGLOCKRrs>;
///Register `DBGLOCKR` writer
pub type W = crate::W<DBGLOCKRrs>;
/**debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored

Value on reset: 180*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBGCFG_LOCKR {
    ///106: Debug configuration register (DBGCR) locked
    Locked = 106,
    ///180: Debug configuration register (DBGCR) unlocked
    Unlocked = 180,
}
impl From<DBGCFG_LOCKR> for u8 {
    #[inline(always)]
    fn from(variant: DBGCFG_LOCKR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBGCFG_LOCKR {
    type Ux = u8;
}
impl crate::IsEnum for DBGCFG_LOCKR {}
///Field `DBGCFG_LOCK` reader - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored
pub type DBGCFG_LOCK_R = crate::FieldReader<DBGCFG_LOCKR>;
impl DBGCFG_LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBGCFG_LOCKR> {
        match self.bits {
            106 => Some(DBGCFG_LOCKR::Locked),
            180 => Some(DBGCFG_LOCKR::Unlocked),
            _ => None,
        }
    }
    ///Debug configuration register (DBGCR) locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DBGCFG_LOCKR::Locked
    }
    ///Debug configuration register (DBGCR) unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DBGCFG_LOCKR::Unlocked
    }
}
/**debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored

Value on reset: 180*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBGCFG_LOCKW {
    ///180: Debug configuration register (DBGCR) unlocked
    Unlocked = 180,
    ///195: Debug configuration register (DBGCR) locked
    Locked = 195,
}
impl From<DBGCFG_LOCKW> for u8 {
    #[inline(always)]
    fn from(variant: DBGCFG_LOCKW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBGCFG_LOCKW {
    type Ux = u8;
}
impl crate::IsEnum for DBGCFG_LOCKW {}
///Field `DBGCFG_LOCK` writer - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored
pub type DBGCFG_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DBGCFG_LOCKW>;
impl<'a, REG> DBGCFG_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Debug configuration register (DBGCR) unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DBGCFG_LOCKW::Unlocked)
    }
    ///Debug configuration register (DBGCR) locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DBGCFG_LOCKW::Locked)
    }
}
impl R {
    ///Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored
    #[inline(always)]
    pub fn dbgcfg_lock(&self) -> DBGCFG_LOCK_R {
        DBGCFG_LOCK_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGLOCKR")
            .field("dbgcfg_lock", &self.dbgcfg_lock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored
    #[inline(always)]
    pub fn dbgcfg_lock(&mut self) -> DBGCFG_LOCK_W<'_, DBGLOCKRrs> {
        DBGCFG_LOCK_W::new(self, 0)
    }
}
/**SBS debug lock register

You can [`read`](crate::Reg::read) this register and get [`dbglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:DBGLOCKR)*/
pub struct DBGLOCKRrs;
impl crate::RegisterSpec for DBGLOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`dbglockr::R`](R) reader structure
impl crate::Readable for DBGLOCKRrs {}
///`write(|w| ..)` method takes [`dbglockr::W`](W) writer structure
impl crate::Writable for DBGLOCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGLOCKR to value 0xb4
impl crate::Resettable for DBGLOCKRrs {
    const RESET_VALUE: u32 = 0xb4;
}
