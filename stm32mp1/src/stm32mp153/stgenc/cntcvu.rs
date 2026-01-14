///Register `CNTCVU` reader
pub type R = crate::R<CNTCVUrs>;
///Register `CNTCVU` writer
pub type W = crate::W<CNTCVUrs>;
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
        f.debug_struct("CNTCVU")
            .field("cntcvu_u_32", &self.cntcvu_u_32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CNTCVU_U_32
    #[inline(always)]
    pub fn cntcvu_u_32(&mut self) -> CNTCVU_U_32_W<'_, CNTCVUrs> {
        CNTCVU_U_32_W::new(self, 0)
    }
}
/**the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntcvu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcvu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:CNTCVU)*/
pub struct CNTCVUrs;
impl crate::RegisterSpec for CNTCVUrs {
    type Ux = u32;
}
///`read()` method returns [`cntcvu::R`](R) reader structure
impl crate::Readable for CNTCVUrs {}
///`write(|w| ..)` method takes [`cntcvu::W`](W) writer structure
impl crate::Writable for CNTCVUrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTCVU to value 0
impl crate::Resettable for CNTCVUrs {}
