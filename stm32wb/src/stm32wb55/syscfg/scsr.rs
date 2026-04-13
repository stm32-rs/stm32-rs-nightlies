///Register `SCSR` reader
pub type R = crate::R<SCSRrs>;
///Register `SCSR` writer
pub type W = crate::W<SCSRrs>;
/**SRAM2 Erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ERW {
    ///1: Start SRAM2 erase operation
    Erase = 1,
}
impl From<SRAM2ERW> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ERW) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2ER` reader - SRAM2 Erase
pub type SRAM2ER_R = crate::BitReader<SRAM2ERW>;
impl SRAM2ER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRAM2ERW> {
        match self.bits {
            true => Some(SRAM2ERW::Erase),
            _ => None,
        }
    }
    ///Start SRAM2 erase operation
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ERW::Erase
    }
}
///Field `SRAM2ER` writer - SRAM2 Erase
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2ERW>;
impl<'a, REG> SRAM2ER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start SRAM2 erase operation
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2ERW::Erase)
    }
}
/**SRAM2 busy by erase operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2BSY {
    ///0: No SRAM2 or PKA RAM erase operation is ongoing
    Idle = 0,
    ///1: SRAM2 and/or PKA RAM erase operation is ongoing
    Busy = 1,
}
impl From<SRAM2BSY> for bool {
    #[inline(always)]
    fn from(variant: SRAM2BSY) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2BSY` reader - SRAM2 busy by erase operation
pub type SRAM2BSY_R = crate::BitReader<SRAM2BSY>;
impl SRAM2BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2BSY {
        match self.bits {
            false => SRAM2BSY::Idle,
            true => SRAM2BSY::Busy,
        }
    }
    ///No SRAM2 or PKA RAM erase operation is ongoing
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SRAM2BSY::Idle
    }
    ///SRAM2 and/or PKA RAM erase operation is ongoing
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAM2BSY::Busy
    }
}
/**CPU2 SRAM fetch (execution) disable.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2RFD {
    ///0: CPU2 fetch from SRAM1, SRAM2a and SRAM2b enabled
    Disabled = 0,
    ///1: CPU2 fetch from SRAM1, SRAM2a and SRAM2b disabled
    Enabled = 1,
}
impl From<C2RFD> for bool {
    #[inline(always)]
    fn from(variant: C2RFD) -> Self {
        variant as u8 != 0
    }
}
///Field `C2RFD` reader - CPU2 SRAM fetch (execution) disable.
pub type C2RFD_R = crate::BitReader<C2RFD>;
impl C2RFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C2RFD {
        match self.bits {
            false => C2RFD::Disabled,
            true => C2RFD::Enabled,
        }
    }
    ///CPU2 fetch from SRAM1, SRAM2a and SRAM2b enabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C2RFD::Disabled
    }
    ///CPU2 fetch from SRAM1, SRAM2a and SRAM2b disabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C2RFD::Enabled
    }
}
///Field `C2RFD` writer - CPU2 SRAM fetch (execution) disable.
pub type C2RFD_W<'a, REG> = crate::BitWriter<'a, REG, C2RFD>;
impl<'a, REG> C2RFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU2 fetch from SRAM1, SRAM2a and SRAM2b enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(C2RFD::Disabled)
    }
    ///CPU2 fetch from SRAM1, SRAM2a and SRAM2b disabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(C2RFD::Enabled)
    }
}
impl R {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 busy by erase operation
    #[inline(always)]
    pub fn sram2bsy(&self) -> SRAM2BSY_R {
        SRAM2BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 31 - CPU2 SRAM fetch (execution) disable.
    #[inline(always)]
    pub fn c2rfd(&self) -> C2RFD_R {
        C2RFD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCSR")
            .field("sram2bsy", &self.sram2bsy())
            .field("sram2er", &self.sram2er())
            .field("c2rfd", &self.c2rfd())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W<'_, SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
    ///Bit 31 - CPU2 SRAM fetch (execution) disable.
    #[inline(always)]
    pub fn c2rfd(&mut self) -> C2RFD_W<'_, SCSRrs> {
        C2RFD_W::new(self, 31)
    }
}
/**SCSR

You can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SCSR)*/
pub struct SCSRrs;
impl crate::RegisterSpec for SCSRrs {
    type Ux = u32;
}
///`read()` method returns [`scsr::R`](R) reader structure
impl crate::Readable for SCSRrs {}
///`write(|w| ..)` method takes [`scsr::W`](W) writer structure
impl crate::Writable for SCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCSR to value 0
impl crate::Resettable for SCSRrs {}
