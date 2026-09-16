///Register `TXBCIE` reader
pub type R = crate::R<TXBCIErs>;
///Register `TXBCIE` writer
pub type W = crate::W<TXBCIErs>;
///Field `CF` reader - Cancellation Finished Interrupt Enable
pub type CF_R = crate::FieldReader<u32>;
///Field `CF` writer - Cancellation Finished Interrupt Enable
pub type CF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Cancellation Finished Interrupt Enable
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCIE").field("cf", &self.cf()).finish()
    }
}
impl W {
    ///Bits 0:31 - Cancellation Finished Interrupt Enable
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W<'_, TXBCIErs> {
        CF_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#FDCAN1:TXBCIE)*/
pub struct TXBCIErs;
impl crate::RegisterSpec for TXBCIErs {
    type Ux = u32;
}
///`read()` method returns [`txbcie::R`](R) reader structure
impl crate::Readable for TXBCIErs {}
///`write(|w| ..)` method takes [`txbcie::W`](W) writer structure
impl crate::Writable for TXBCIErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBCIE to value 0
impl crate::Resettable for TXBCIErs {}
