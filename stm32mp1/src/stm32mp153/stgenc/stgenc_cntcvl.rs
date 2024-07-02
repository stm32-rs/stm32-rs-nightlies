///Register `STGENC_CNTCVL` reader
pub type R = crate::R<STGENC_CNTCVLrs>;
///Register `STGENC_CNTCVL` writer
pub type W = crate::W<STGENC_CNTCVLrs>;
///Field `CNTCVL_L_32` reader - CNTCVL_L_32
pub type CNTCVL_L_32_R = crate::FieldReader<u32>;
///Field `CNTCVL_L_32` writer - CNTCVL_L_32
pub type CNTCVL_L_32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CNTCVL_L_32
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_CNTCVL")
            .field("cntcvl_l_32", &self.cntcvl_l_32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CNTCVL_L_32
    #[inline(always)]
    #[must_use]
    pub fn cntcvl_l_32(&mut self) -> CNTCVL_L_32_W<STGENC_CNTCVLrs> {
        CNTCVL_L_32_W::new(self, 0)
    }
}
/**the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntcvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntcvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTCVL)*/
pub struct STGENC_CNTCVLrs;
impl crate::RegisterSpec for STGENC_CNTCVLrs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cntcvl::R`](R) reader structure
impl crate::Readable for STGENC_CNTCVLrs {}
///`write(|w| ..)` method takes [`stgenc_cntcvl::W`](W) writer structure
impl crate::Writable for STGENC_CNTCVLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STGENC_CNTCVL to value 0
impl crate::Resettable for STGENC_CNTCVLrs {
    const RESET_VALUE: u32 = 0;
}
