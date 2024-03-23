#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2rs>;
#[doc = "Field `FLASH_RDY` reader - Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory."]
pub type FLASH_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory."]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PWR status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2rs {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0;
}
