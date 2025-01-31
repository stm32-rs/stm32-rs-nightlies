///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `FLASH_RDY` reader - Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory.
pub type FLASH_RDY_R = crate::BitReader;
/**Field `PVM_VDDIO2_OUT` reader - V<sub>DDIO2</sub> supply voltage monitoring output flag This flag indicates the readiness of the V<sub>DDIO2</sub> supply voltage (excess of 1.2 V). The flag is cleared when the PVM of V<sub>DDIO2</sub> is disabled (PVM_VDDIO2\[0\]
= 0). Note: Only applicable on STM32C071xx, reserved on the other products.*/
pub type PVM_VDDIO2_OUT_R = crate::BitReader;
impl R {
    ///Bit 7 - Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory.
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    /**Bit 13 - V<sub>DDIO2</sub> supply voltage monitoring output flag This flag indicates the readiness of the V<sub>DDIO2</sub> supply voltage (excess of 1.2 V). The flag is cleared when the PVM of V<sub>DDIO2</sub> is disabled (PVM_VDDIO2\[0\]
    = 0). Note: Only applicable on STM32C071xx, reserved on the other products.*/
    #[inline(always)]
    pub fn pvm_vddio2_out(&self) -> PVM_VDDIO2_OUT_R {
        PVM_VDDIO2_OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("flash_rdy", &self.flash_rdy())
            .field("pvm_vddio2_out", &self.pvm_vddio2_out())
            .finish()
    }
}
/**PWR status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0;
}
