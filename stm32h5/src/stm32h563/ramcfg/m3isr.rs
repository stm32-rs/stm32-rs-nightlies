///Register `M3ISR` reader
pub type R = crate::R<M3ISRrs>;
///Field `SEDC` reader - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
pub type SEDC_R = crate::BitReader;
///Field `DED` reader - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
pub type DED_R = crate::BitReader;
///Field `SRAMBUSY` reader - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or readout protection regression. Refer to Table 18: Internal SRAMs features.
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
    #[inline(always)]
    pub fn sedc(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or readout protection regression. Refer to Table 18: Internal SRAMs features.
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3ISR")
            .field("sedc", &self.sedc())
            .field("ded", &self.ded())
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG memory interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m3isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RAMCFG:M3ISR)*/
pub struct M3ISRrs;
impl crate::RegisterSpec for M3ISRrs {
    type Ux = u32;
}
///`read()` method returns [`m3isr::R`](R) reader structure
impl crate::Readable for M3ISRrs {}
///`reset()` method sets M3ISR to value 0
impl crate::Resettable for M3ISRrs {}
