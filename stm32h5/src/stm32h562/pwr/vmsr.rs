#[doc = "Register `VMSR` reader"]
pub type R = crate::R<VMSRrs>;
#[doc = "analog voltage detector output on V&lt;sub>DDA&lt;/sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDOR {
    #[doc = "0: VDDA is equal or higher than the AVD threshold selected with the ALS\\[2:0\\]
bits"]
    AboveThreshold = 0,
    #[doc = "1: VDDA is lower than the AVD threshold selected with the ALS\\[2:0\\]
bits"]
    BelowThreshold = 1,
}
impl From<AVDOR> for bool {
    #[inline(always)]
    fn from(variant: AVDOR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVDO` reader - analog voltage detector output on V&lt;sub>DDA&lt;/sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
pub type AVDO_R = crate::BitReader<AVDOR>;
impl AVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AVDOR {
        match self.bits {
            false => AVDOR::AboveThreshold,
            true => AVDOR::BelowThreshold,
        }
    }
    #[doc = "VDDA is equal or higher than the AVD threshold selected with the ALS\\[2:0\\]
bits"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == AVDOR::AboveThreshold
    }
    #[doc = "VDDA is lower than the AVD threshold selected with the ALS\\[2:0\\]
bits"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == AVDOR::BelowThreshold
    }
}
#[doc = "voltage detector output on V&lt;sub>DDIO2&lt;/sub> This bit is set and cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDIO2RDYR {
    #[doc = "0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor"]
    BelowThreshold = 0,
    #[doc = "1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor"]
    AboveThreshold = 1,
}
impl From<VDDIO2RDYR> for bool {
    #[inline(always)]
    fn from(variant: VDDIO2RDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDIO2RDY` reader - voltage detector output on V&lt;sub>DDIO2&lt;/sub> This bit is set and cleared by hardware."]
pub type VDDIO2RDY_R = crate::BitReader<VDDIO2RDYR>;
impl VDDIO2RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDDIO2RDYR {
        match self.bits {
            false => VDDIO2RDYR::BelowThreshold,
            true => VDDIO2RDYR::AboveThreshold,
        }
    }
    #[doc = "VDDIO2 is below the threshold of the VDDIO2 voltage monitor"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VDDIO2RDYR::BelowThreshold
    }
    #[doc = "VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VDDIO2RDYR::AboveThreshold
    }
}
#[doc = "programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDOR {
    #[doc = "0: VDD is equal or higher than the PVD threshold selected through the PLS\\[2:0\\]
bits."]
    AboveThreshold = 0,
    #[doc = "1: VDD is lower than the PVD threshold selected through the PLS\\[2:0\\]
bits"]
    BelowThreshold = 1,
}
impl From<PVDOR> for bool {
    #[inline(always)]
    fn from(variant: PVDOR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
pub type PVDO_R = crate::BitReader<PVDOR>;
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDOR {
        match self.bits {
            false => PVDOR::AboveThreshold,
            true => PVDOR::BelowThreshold,
        }
    }
    #[doc = "VDD is equal or higher than the PVD threshold selected through the PLS\\[2:0\\]
bits."]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == PVDOR::AboveThreshold
    }
    #[doc = "VDD is lower than the PVD threshold selected through the PLS\\[2:0\\]
bits"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == PVDOR::BelowThreshold
    }
}
#[doc = "Field `USB33RDY` reader - V&lt;sub>DDUSB&lt;/sub> ready"]
pub type USB33RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 19 - analog voltage detector output on V&lt;sub>DDA&lt;/sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - voltage detector output on V&lt;sub>DDIO2&lt;/sub> This bit is set and cleared by hardware."]
    #[inline(always)]
    pub fn vddio2rdy(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - V&lt;sub>DDUSB&lt;/sub> ready"]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "PWR voltage monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMSRrs;
impl crate::RegisterSpec for VMSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmsr::R`](R) reader structure"]
impl crate::Readable for VMSRrs {}
#[doc = "`reset()` method sets VMSR to value 0"]
impl crate::Resettable for VMSRrs {
    const RESET_VALUE: u32 = 0;
}
