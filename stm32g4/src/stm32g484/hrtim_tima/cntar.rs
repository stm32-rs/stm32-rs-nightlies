///Register `CNTAR` reader
pub type R = crate::R<CNTARrs>;
///Register `CNTAR` writer
pub type W = crate::W<CNTARrs>;
///Field `CNTx` reader - Timerx Counter value
pub type CNTX_R = crate::FieldReader<u16>;
///Field `CNTx` writer - Timerx Counter value
pub type CNTX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTAR").field("cntx", &self.cntx()).finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<CNTARrs> {
        CNTX_W::new(self, 0)
    }
}
/**Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#HRTIM_TIMA:CNTAR)*/
pub struct CNTARrs;
impl crate::RegisterSpec for CNTARrs {
    type Ux = u32;
}
///`read()` method returns [`cntar::R`](R) reader structure
impl crate::Readable for CNTARrs {}
///`write(|w| ..)` method takes [`cntar::W`](W) writer structure
impl crate::Writable for CNTARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNTAR to value 0
impl crate::Resettable for CNTARrs {
    const RESET_VALUE: u32 = 0;
}
