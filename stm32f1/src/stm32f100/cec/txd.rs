///Register `TXD` reader
pub type R = crate::R<TXDrs>;
///Register `TXD` writer
pub type W = crate::W<TXDrs>;
///Field `TXD` reader - Tx Data register
pub type TXD_R = crate::FieldReader;
///Field `TXD` writer - Tx Data register
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Tx Data register
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXD").field("txd", &self.txd()).finish()
    }
}
impl W {
    ///Bits 0:7 - Tx Data register
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W<'_, TXDrs> {
        TXD_W::new(self, 0)
    }
}
/**CEC Tx data register

You can [`read`](crate::Reg::read) this register and get [`txd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:TXD)*/
pub struct TXDrs;
impl crate::RegisterSpec for TXDrs {
    type Ux = u32;
}
///`read()` method returns [`txd::R`](R) reader structure
impl crate::Readable for TXDrs {}
///`write(|w| ..)` method takes [`txd::W`](W) writer structure
impl crate::Writable for TXDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXD to value 0
impl crate::Resettable for TXDrs {}
