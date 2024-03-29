#[doc = "Register `M6ISR` reader"]
pub type R = crate::R<M6ISRrs>;
#[doc = "Field `SEDC` reader - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1, SRAM4 and SRAM5 interrupt status registers."]
pub type SEDC_R = crate::BitReader;
#[doc = "Field `DED` reader - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1, SRAM4 and SRAM5 interrupt status registers."]
pub type DED_R = crate::BitReader;
#[doc = "Field `SRAMBUSY` reader - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or readout protection regression. Refer to ."]
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1, SRAM4 and SRAM5 interrupt status registers."]
    #[inline(always)]
    pub fn sedc(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1, SRAM4 and SRAM5 interrupt status registers."]
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or readout protection regression. Refer to ."]
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m6isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M6ISRrs;
impl crate::RegisterSpec for M6ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m6isr::R`](R) reader structure"]
impl crate::Readable for M6ISRrs {}
#[doc = "`reset()` method sets M6ISR to value 0"]
impl crate::Resettable for M6ISRrs {
    const RESET_VALUE: u32 = 0;
}
