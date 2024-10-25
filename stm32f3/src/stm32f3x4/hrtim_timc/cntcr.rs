///Register `CNTCR` reader
pub type R = crate::R<CNTCRrs>;
///Register `CNTCR` writer
pub type W = crate::W<CNTCRrs>;
///Field `CNTx` reader - Timerx Counter value
pub type CNTX_R = crate::FieldReader<u16>;
///Field `CNTx` writer - Timerx Counter value
pub type CNTX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTCR").field("cntx", &self.cntx()).finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<CNTCRrs> {
        CNTX_W::new(self, 0)
    }
}
/**Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMC:CNTCR)*/
pub struct CNTCRrs;
impl crate::RegisterSpec for CNTCRrs {
    type Ux = u32;
}
///`read()` method returns [`cntcr::R`](R) reader structure
impl crate::Readable for CNTCRrs {}
///`write(|w| ..)` method takes [`cntcr::W`](W) writer structure
impl crate::Writable for CNTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNTCR to value 0
impl crate::Resettable for CNTCRrs {
    const RESET_VALUE: u32 = 0;
}
