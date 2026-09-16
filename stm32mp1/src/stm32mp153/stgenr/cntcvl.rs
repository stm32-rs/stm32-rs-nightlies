///Register `CNTCVL` reader
pub type R = crate::R<CNTCVLrs>;
///Field `CNTCVL_L_32` reader - CNTCVL_L_32
pub type CNTCVL_L_32_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CNTCVL_L_32
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTCVL")
            .field("cntcvl_l_32", &self.cntcvl_l_32())
            .finish()
    }
}
/**the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntcvl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENR:CNTCVL)*/
pub struct CNTCVLrs;
impl crate::RegisterSpec for CNTCVLrs {
    type Ux = u32;
}
///`read()` method returns [`cntcvl::R`](R) reader structure
impl crate::Readable for CNTCVLrs {}
///`reset()` method sets CNTCVL to value 0
impl crate::Resettable for CNTCVLrs {}
