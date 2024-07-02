///Register `CNTDR` reader
pub type R = crate::R<CNTDRrs>;
///Register `CNTDR` writer
pub type W = crate::W<CNTDRrs>;
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
        f.debug_struct("CNTDR").field("cntx", &self.cntx()).finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<CNTDRrs> {
        CNTX_W::new(self, 0)
    }
}
/**Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIMD:CNTDR)*/
pub struct CNTDRrs;
impl crate::RegisterSpec for CNTDRrs {
    type Ux = u32;
}
///`read()` method returns [`cntdr::R`](R) reader structure
impl crate::Readable for CNTDRrs {}
///`write(|w| ..)` method takes [`cntdr::W`](W) writer structure
impl crate::Writable for CNTDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNTDR to value 0
impl crate::Resettable for CNTDRrs {
    const RESET_VALUE: u32 = 0;
}
