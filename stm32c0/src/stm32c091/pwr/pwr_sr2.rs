///Register `PWR_SR2` reader
pub type R = crate::R<PWR_SR2rs>;
/**Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_RDY {
    ///0: Flash memory in power-down
    B0x0 = 0,
    ///1: Flash memory ready to be accessed
    B0x1 = 1,
}
impl From<FLASH_RDY> for bool {
    #[inline(always)]
    fn from(variant: FLASH_RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASH_RDY` reader - Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory.
pub type FLASH_RDY_R = crate::BitReader<FLASH_RDY>;
impl FLASH_RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASH_RDY {
        match self.bits {
            false => FLASH_RDY::B0x0,
            true => FLASH_RDY::B0x1,
        }
    }
    ///Flash memory in power-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASH_RDY::B0x0
    }
    ///Flash memory ready to be accessed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASH_RDY::B0x1
    }
}
/**V<sub>DDIO2</sub> supply voltage monitoring output flag This flag indicates the readiness of the V<sub>DDIO2</sub> supply voltage (excess of 1.2 V). The flag is cleared when the PVM of V<sub>DDIO2</sub> is disabled (PVM_VDDIO2\[0\] = 0). Note: Only applicable on STM32C071xx, reserved on the other products.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVM_VDDIO2_OUT {
    ///0: Not ready
    B0x0 = 0,
    ///1: Ready
    B0x1 = 1,
}
impl From<PVM_VDDIO2_OUT> for bool {
    #[inline(always)]
    fn from(variant: PVM_VDDIO2_OUT) -> Self {
        variant as u8 != 0
    }
}
///Field `PVM_VDDIO2_OUT` reader - V<sub>DDIO2</sub> supply voltage monitoring output flag This flag indicates the readiness of the V<sub>DDIO2</sub> supply voltage (excess of 1.2 V). The flag is cleared when the PVM of V<sub>DDIO2</sub> is disabled (PVM_VDDIO2\[0\] = 0). Note: Only applicable on STM32C071xx, reserved on the other products.
pub type PVM_VDDIO2_OUT_R = crate::BitReader<PVM_VDDIO2_OUT>;
impl PVM_VDDIO2_OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVM_VDDIO2_OUT {
        match self.bits {
            false => PVM_VDDIO2_OUT::B0x0,
            true => PVM_VDDIO2_OUT::B0x1,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVM_VDDIO2_OUT::B0x0
    }
    ///Ready
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVM_VDDIO2_OUT::B0x1
    }
}
impl R {
    ///Bit 7 - Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory.
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - V<sub>DDIO2</sub> supply voltage monitoring output flag This flag indicates the readiness of the V<sub>DDIO2</sub> supply voltage (excess of 1.2 V). The flag is cleared when the PVM of V<sub>DDIO2</sub> is disabled (PVM_VDDIO2\[0\] = 0). Note: Only applicable on STM32C071xx, reserved on the other products.
    #[inline(always)]
    pub fn pvm_vddio2_out(&self) -> PVM_VDDIO2_OUT_R {
        PVM_VDDIO2_OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SR2")
            .field("flash_rdy", &self.flash_rdy())
            .field("pvm_vddio2_out", &self.pvm_vddio2_out())
            .finish()
    }
}
/**PWR status register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_SR2)*/
pub struct PWR_SR2rs;
impl crate::RegisterSpec for PWR_SR2rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_sr2::R`](R) reader structure
impl crate::Readable for PWR_SR2rs {}
///`reset()` method sets PWR_SR2 to value 0
impl crate::Resettable for PWR_SR2rs {}
