///Register `WRPSGN2R_CUR` reader
pub type R = crate::R<WRPSGN2R_CURrs>;
///Field `WRPSG2` reader - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)
pub type WRPSG2_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPSGN2R_CUR")
            .field("wrpsg2", &self.wrpsg2())
            .finish()
    }
}
/**FLASH write sector protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN2R_CUR)*/
pub struct WRPSGN2R_CURrs;
impl crate::RegisterSpec for WRPSGN2R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`wrpsgn2r_cur::R`](R) reader structure
impl crate::Readable for WRPSGN2R_CURrs {}
///`reset()` method sets WRPSGN2R_CUR to value 0
impl crate::Resettable for WRPSGN2R_CURrs {}
