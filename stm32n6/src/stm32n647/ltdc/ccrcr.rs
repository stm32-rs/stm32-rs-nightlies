///Register `CCRCR` reader
pub type R = crate::R<CCRCRrs>;
///Field `CCRC` reader - computed CRC of frame
pub type CCRC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - computed CRC of frame
    #[inline(always)]
    pub fn ccrc(&self) -> CCRC_R {
        CCRC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCRCR").field("ccrc", &self.ccrc()).finish()
    }
}
/**LTDC computed CRC register

You can [`read`](crate::Reg::read) this register and get [`ccrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#LTDC:CCRCR)*/
pub struct CCRCRrs;
impl crate::RegisterSpec for CCRCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccrcr::R`](R) reader structure
impl crate::Readable for CCRCRrs {}
///`reset()` method sets CCRCR to value 0
impl crate::Resettable for CCRCRrs {}
