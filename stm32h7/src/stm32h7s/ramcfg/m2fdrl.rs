///Register `M2FDRL` reader
pub type R = crate::R<M2FDRLrs>;
///Field `FDATAL` reader - Failing data low When an ECC error occurs the FDATAL bitfield contains the LSB part of the data that generated the error. For 32-bit word SRAM, this bitfield contains the full memory word that generated the error.
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing data low When an ECC error occurs the FDATAL bitfield contains the LSB part of the data that generated the error. For 32-bit word SRAM, this bitfield contains the full memory word that generated the error.
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor 2 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m2fdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RAMCFG:M2FDRL)*/
pub struct M2FDRLrs;
impl crate::RegisterSpec for M2FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m2fdrl::R`](R) reader structure
impl crate::Readable for M2FDRLrs {}
///`reset()` method sets M2FDRL to value 0
impl crate::Resettable for M2FDRLrs {
    const RESET_VALUE: u32 = 0;
}