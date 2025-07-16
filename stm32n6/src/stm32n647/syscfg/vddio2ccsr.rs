///Register `VDDIO2CCSR` reader
pub type R = crate::R<VDDIO2CCSRrs>;
///Field `ANSRC` reader - This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.
pub type ANSRC_R = crate::FieldReader;
///Field `APSRC` reader - This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.
pub type APSRC_R = crate::FieldReader;
///Field `READY` reader - Provides the compensation cell status of I/Os supplied by VDDIOx
pub type READY_R = crate::BitReader;
impl R {
    ///Bits 0:3 - This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.
    #[inline(always)]
    pub fn ansrc(&self) -> ANSRC_R {
        ANSRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.
    #[inline(always)]
    pub fn apsrc(&self) -> APSRC_R {
        APSRC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Provides the compensation cell status of I/Os supplied by VDDIOx
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDIO2CCSR")
            .field("ansrc", &self.ansrc())
            .field("apsrc", &self.apsrc())
            .field("ready", &self.ready())
            .finish()
    }
}
/**SYSCFG VDDIO2 compensation cell status register

You can [`read`](crate::Reg::read) this register and get [`vddio2ccsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SYSCFG:VDDIO2CCSR)*/
pub struct VDDIO2CCSRrs;
impl crate::RegisterSpec for VDDIO2CCSRrs {
    type Ux = u32;
}
///`read()` method returns [`vddio2ccsr::R`](R) reader structure
impl crate::Readable for VDDIO2CCSRrs {}
///`reset()` method sets VDDIO2CCSR to value 0
impl crate::Resettable for VDDIO2CCSRrs {}
