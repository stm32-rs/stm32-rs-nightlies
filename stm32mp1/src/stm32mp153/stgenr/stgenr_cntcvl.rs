///Register `STGENR_CNTCVL` reader
pub type R = crate::R<STGENR_CNTCVLrs>;
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
        f.debug_struct("STGENR_CNTCVL")
            .field("cntcvl_l_32", &self.cntcvl_l_32())
            .finish()
    }
}
/**the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenr_cntcvl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENR:STGENR_CNTCVL)*/
pub struct STGENR_CNTCVLrs;
impl crate::RegisterSpec for STGENR_CNTCVLrs {
    type Ux = u32;
}
///`read()` method returns [`stgenr_cntcvl::R`](R) reader structure
impl crate::Readable for STGENR_CNTCVLrs {}
///`reset()` method sets STGENR_CNTCVL to value 0
impl crate::Resettable for STGENR_CNTCVLrs {
    const RESET_VALUE: u32 = 0;
}
