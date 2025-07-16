///Register `CMPCR` reader
pub type R = crate::R<CMPCRrs>;
/**Compensation cell power-down

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP_PD {
    ///0: I/O compensation cell power-down mode
    PowerDown = 0,
    ///1: I/O compensation cell enabled
    Enabled = 1,
}
impl From<CMP_PD> for bool {
    #[inline(always)]
    fn from(variant: CMP_PD) -> Self {
        variant as u8 != 0
    }
}
///Field `CMP_PD` reader - Compensation cell power-down
pub type CMP_PD_R = crate::BitReader<CMP_PD>;
impl CMP_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP_PD {
        match self.bits {
            false => CMP_PD::PowerDown,
            true => CMP_PD::Enabled,
        }
    }
    ///I/O compensation cell power-down mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == CMP_PD::PowerDown
    }
    ///I/O compensation cell enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP_PD::Enabled
    }
}
/**READY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY {
    ///0: I/O compensation cell not ready
    NotReady = 0,
    ///1: I/O compensation cell ready
    Ready = 1,
}
impl From<READY> for bool {
    #[inline(always)]
    fn from(variant: READY) -> Self {
        variant as u8 != 0
    }
}
///Field `READY` reader - READY
pub type READY_R = crate::BitReader<READY>;
impl READY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> READY {
        match self.bits {
            false => READY::NotReady,
            true => READY::Ready,
        }
    }
    ///I/O compensation cell not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == READY::NotReady
    }
    ///I/O compensation cell ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY::Ready
    }
}
impl R {
    ///Bit 0 - Compensation cell power-down
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - READY
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPCR")
            .field("ready", &self.ready())
            .field("cmp_pd", &self.cmp_pd())
            .finish()
    }
}
/**Compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`cmpcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#SYSCFG:CMPCR)*/
pub struct CMPCRrs;
impl crate::RegisterSpec for CMPCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmpcr::R`](R) reader structure
impl crate::Readable for CMPCRrs {}
///`reset()` method sets CMPCR to value 0
impl crate::Resettable for CMPCRrs {}
