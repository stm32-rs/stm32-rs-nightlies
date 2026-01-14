///Register `CNTCVU` reader
pub type R = crate::R<CNTCVUrs>;
///Field `CNTCVU_U_32` reader - CNTCVU_U_32
pub type CNTCVU_U_32_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CNTCVU_U_32
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTCVU")
            .field("cntcvu_u_32", &self.cntcvu_u_32())
            .finish()
    }
}
/**the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntcvu::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENR:CNTCVU)*/
pub struct CNTCVUrs;
impl crate::RegisterSpec for CNTCVUrs {
    type Ux = u32;
}
///`read()` method returns [`cntcvu::R`](R) reader structure
impl crate::Readable for CNTCVUrs {}
///`reset()` method sets CNTCVU to value 0
impl crate::Resettable for CNTCVUrs {}
