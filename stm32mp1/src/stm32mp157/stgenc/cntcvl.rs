///Register `CNTCVL` reader
pub type R = crate::R<CNTCVLrs>;
///Register `CNTCVL` writer
pub type W = crate::W<CNTCVLrs>;
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
        f.debug_struct("CNTCVL")
            .field("cntcvl_l_32", &self.cntcvl_l_32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CNTCVL_L_32
    #[inline(always)]
    pub fn cntcvl_l_32(&mut self) -> CNTCVL_L_32_W<'_, CNTCVLrs> {
        CNTCVL_L_32_W::new(self, 0)
    }
}
/**the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntcvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTCVL)*/
pub struct CNTCVLrs;
impl crate::RegisterSpec for CNTCVLrs {
    type Ux = u32;
}
///`read()` method returns [`cntcvl::R`](R) reader structure
impl crate::Readable for CNTCVLrs {}
///`write(|w| ..)` method takes [`cntcvl::W`](W) writer structure
impl crate::Writable for CNTCVLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTCVL to value 0
impl crate::Resettable for CNTCVLrs {}
