///Register `CNTFID0` reader
pub type R = crate::R<CNTFID0rs>;
///Register `CNTFID0` writer
pub type W = crate::W<CNTFID0rs>;
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
        f.debug_struct("CNTFID0")
            .field("freq", &self.freq())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - FREQ
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W<'_, CNTFID0rs> {
        FREQ_W::new(self, 0)
    }
}
/**the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntfid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntfid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:CNTFID0)*/
pub struct CNTFID0rs;
impl crate::RegisterSpec for CNTFID0rs {
    type Ux = u32;
}
///`read()` method returns [`cntfid0::R`](R) reader structure
impl crate::Readable for CNTFID0rs {}
///`write(|w| ..)` method takes [`cntfid0::W`](W) writer structure
impl crate::Writable for CNTFID0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTFID0 to value 0
impl crate::Resettable for CNTFID0rs {}
