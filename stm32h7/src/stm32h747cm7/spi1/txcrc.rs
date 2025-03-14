///Register `TXCRC` reader
pub type R = crate::R<TXCRCrs>;
///Register `TXCRC` writer
pub type W = crate::W<TXCRCrs>;
///Field `TXCRC` reader - CRC register for transmitter
pub type TXCRC_R = crate::FieldReader<u32>;
///Field `TXCRC` writer - CRC register for transmitter
pub type TXCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRC register for transmitter
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXCRC")
            .field("txcrc", &self.txcrc())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRC register for transmitter
    #[inline(always)]
    pub fn txcrc(&mut self) -> TXCRC_W<TXCRCrs> {
        TXCRC_W::new(self, 0)
    }
}
/**Transmitter CRC Register

You can [`read`](crate::Reg::read) this register and get [`txcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#SPI1:TXCRC)*/
pub struct TXCRCrs;
impl crate::RegisterSpec for TXCRCrs {
    type Ux = u32;
}
///`read()` method returns [`txcrc::R`](R) reader structure
impl crate::Readable for TXCRCrs {}
///`write(|w| ..)` method takes [`txcrc::W`](W) writer structure
impl crate::Writable for TXCRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXCRC to value 0
impl crate::Resettable for TXCRCrs {}
