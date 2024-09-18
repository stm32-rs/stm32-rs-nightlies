///Register `STGENC_CNTFID0` reader
pub type R = crate::R<STGENC_CNTFID0rs>;
///Register `STGENC_CNTFID0` writer
pub type W = crate::W<STGENC_CNTFID0rs>;
///Field `FREQ` reader - FREQ
pub type FREQ_R = crate::FieldReader<u32>;
///Field `FREQ` writer - FREQ
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - FREQ
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_CNTFID0")
            .field("freq", &self.freq())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - FREQ
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<STGENC_CNTFID0rs> {
        FREQ_W::new(self, 0)
    }
}
/**the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntfid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntfid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:STGENC_CNTFID0)*/
pub struct STGENC_CNTFID0rs;
impl crate::RegisterSpec for STGENC_CNTFID0rs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cntfid0::R`](R) reader structure
impl crate::Readable for STGENC_CNTFID0rs {}
///`write(|w| ..)` method takes [`stgenc_cntfid0::W`](W) writer structure
impl crate::Writable for STGENC_CNTFID0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STGENC_CNTFID0 to value 0
impl crate::Resettable for STGENC_CNTFID0rs {
    const RESET_VALUE: u32 = 0;
}
