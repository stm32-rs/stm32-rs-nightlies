///Register `VMSR` reader
pub type R = crate::R<VMSRrs>;
/**analog voltage detector output on V sub DDA /sub This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDOR {
    ///0: VDDA is equal or higher than the AVD threshold selected with the ALS\[2:0\] bits
    AboveThreshold = 0,
    ///1: VDDA is lower than the AVD threshold selected with the ALS\[2:0\] bits
    BelowThreshold = 1,
}
impl From<AVDOR> for bool {
    #[inline(always)]
    fn from(variant: AVDOR) -> Self {
        variant as u8 != 0
    }
}
///Field `AVDO` reader - analog voltage detector output on V sub DDA /sub This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.
pub type AVDO_R = crate::BitReader<AVDOR>;
impl AVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AVDOR {
        match self.bits {
            false => AVDOR::AboveThreshold,
            true => AVDOR::BelowThreshold,
        }
    }
    ///VDDA is equal or higher than the AVD threshold selected with the ALS\[2:0\] bits
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == AVDOR::AboveThreshold
    }
    ///VDDA is lower than the AVD threshold selected with the ALS\[2:0\] bits
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == AVDOR::BelowThreshold
    }
}
/**voltage detector output on V sub DDIO2 /sub This bit is set and cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDIO2RDYR {
    ///0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor
    BelowThreshold = 0,
    ///1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor
    AboveThreshold = 1,
}
impl From<VDDIO2RDYR> for bool {
    #[inline(always)]
    fn from(variant: VDDIO2RDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDIO2RDY` reader - voltage detector output on V sub DDIO2 /sub This bit is set and cleared by hardware.
pub type VDDIO2RDY_R = crate::BitReader<VDDIO2RDYR>;
impl VDDIO2RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDIO2RDYR {
        match self.bits {
            false => VDDIO2RDYR::BelowThreshold,
            true => VDDIO2RDYR::AboveThreshold,
        }
    }
    ///VDDIO2 is below the threshold of the VDDIO2 voltage monitor
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VDDIO2RDYR::BelowThreshold
    }
    ///VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VDDIO2RDYR::AboveThreshold
    }
}
/**programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDOR {
    ///0: VDD is equal or higher than the PVD threshold selected through the PLS\[2:0\] bits.
    AboveThreshold = 0,
    ///1: VDD is lower than the PVD threshold selected through the PLS\[2:0\] bits
    BelowThreshold = 1,
}
impl From<PVDOR> for bool {
    #[inline(always)]
    fn from(variant: PVDOR) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.
pub type PVDO_R = crate::BitReader<PVDOR>;
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDOR {
        match self.bits {
            false => PVDOR::AboveThreshold,
            true => PVDOR::BelowThreshold,
        }
    }
    ///VDD is equal or higher than the PVD threshold selected through the PLS\[2:0\] bits.
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == PVDOR::AboveThreshold
    }
    ///VDD is lower than the PVD threshold selected through the PLS\[2:0\] bits
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == PVDOR::BelowThreshold
    }
}
///Field `USB33RDY` reader - V sub DDUSB /sub ready
pub type USB33RDY_R = crate::BitReader;
impl R {
    ///Bit 19 - analog voltage detector output on V sub DDA /sub This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - voltage detector output on V sub DDIO2 /sub This bit is set and cleared by hardware.
    #[inline(always)]
    pub fn vddio2rdy(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - V sub DDUSB /sub ready
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMSR")
            .field("avdo", &self.avdo())
            .field("vddio2rdy", &self.vddio2rdy())
            .field("pvdo", &self.pvdo())
            .field("usb33rdy", &self.usb33rdy())
            .finish()
    }
}
/**PWR voltage monitor status register

You can [`read`](crate::Reg::read) this register and get [`vmsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#PWR:VMSR)*/
pub struct VMSRrs;
impl crate::RegisterSpec for VMSRrs {
    type Ux = u32;
}
///`read()` method returns [`vmsr::R`](R) reader structure
impl crate::Readable for VMSRrs {}
///`reset()` method sets VMSR to value 0
impl crate::Resettable for VMSRrs {}
