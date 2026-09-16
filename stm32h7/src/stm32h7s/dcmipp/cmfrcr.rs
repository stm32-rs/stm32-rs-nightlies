///Register `CMFRCR` reader
pub type R = crate::R<CMFRCRrs>;
///Field `FRMCNT` reader - Frame counter, read-only, loops around. Incremented following VSYNC detection mapped to the pipe configured into bits PSFC\[1:0\] of the DCMIPP_CMCR register. The counter is cleared using the CRC bit in the DCMIPP_CMCR register.
pub type FRMCNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Frame counter, read-only, loops around. Incremented following VSYNC detection mapped to the pipe configured into bits PSFC\[1:0\] of the DCMIPP_CMCR register. The counter is cleared using the CRC bit in the DCMIPP_CMCR register.
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMFRCR")
            .field("frmcnt", &self.frmcnt())
            .finish()
    }
}
/**DCMIPP common frame counter register

You can [`read`](crate::Reg::read) this register and get [`cmfrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:CMFRCR)*/
pub struct CMFRCRrs;
impl crate::RegisterSpec for CMFRCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmfrcr::R`](R) reader structure
impl crate::Readable for CMFRCRrs {}
///`reset()` method sets CMFRCR to value 0
impl crate::Resettable for CMFRCRrs {}
