///Register `STGENC_CNTCVU` reader
pub type R = crate::R<STGENC_CNTCVUrs>;
///Register `STGENC_CNTCVU` writer
pub type W = crate::W<STGENC_CNTCVUrs>;
///Field `CNTCVU_U_32` reader - CNTCVU_U_32
pub type CNTCVU_U_32_R = crate::FieldReader<u32>;
///Field `CNTCVU_U_32` writer - CNTCVU_U_32
pub type CNTCVU_U_32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CNTCVU_U_32
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_CNTCVU")
            .field("cntcvu_u_32", &self.cntcvu_u_32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CNTCVU_U_32
    #[inline(always)]
    #[must_use]
    pub fn cntcvu_u_32(&mut self) -> CNTCVU_U_32_W<STGENC_CNTCVUrs> {
        CNTCVU_U_32_W::new(self, 0)
    }
}
/**the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntcvu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntcvu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTCVU)*/
pub struct STGENC_CNTCVUrs;
impl crate::RegisterSpec for STGENC_CNTCVUrs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cntcvu::R`](R) reader structure
impl crate::Readable for STGENC_CNTCVUrs {}
///`write(|w| ..)` method takes [`stgenc_cntcvu::W`](W) writer structure
impl crate::Writable for STGENC_CNTCVUrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STGENC_CNTCVU to value 0
impl crate::Resettable for STGENC_CNTCVUrs {
    const RESET_VALUE: u32 = 0;
}
