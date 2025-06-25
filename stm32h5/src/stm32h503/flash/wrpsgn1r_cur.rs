///Register `WRPSGN1R_CUR` reader
pub type R = crate::R<WRPSGN1R_CURrs>;
///Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
pub type WRPSG1_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPSGN1R_CUR")
            .field("wrpsg1", &self.wrpsg1())
            .finish()
    }
}
/**FLASH write sector protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN1R_CUR)*/
pub struct WRPSGN1R_CURrs;
impl crate::RegisterSpec for WRPSGN1R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`wrpsgn1r_cur::R`](R) reader structure
impl crate::Readable for WRPSGN1R_CURrs {}
///`reset()` method sets WRPSGN1R_CUR to value 0
impl crate::Resettable for WRPSGN1R_CURrs {}
