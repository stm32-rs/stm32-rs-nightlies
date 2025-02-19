///Register `RXCRC` reader
pub type R = crate::R<RXCRCrs>;
///Register `RXCRC` writer
pub type W = crate::W<RXCRCrs>;
///Field `RXCRC` reader - CRC register for receiver
pub type RXCRC_R = crate::FieldReader<u32>;
///Field `RXCRC` writer - CRC register for receiver
pub type RXCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRC register for receiver
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXCRC")
            .field("rxcrc", &self.rxcrc())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRC register for receiver
    #[inline(always)]
    pub fn rxcrc(&mut self) -> RXCRC_W<RXCRCrs> {
        RXCRC_W::new(self, 0)
    }
}
/**Receiver CRC Register

You can [`read`](crate::Reg::read) this register and get [`rxcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SPI1:RXCRC)*/
pub struct RXCRCrs;
impl crate::RegisterSpec for RXCRCrs {
    type Ux = u32;
}
///`read()` method returns [`rxcrc::R`](R) reader structure
impl crate::Readable for RXCRCrs {}
///`write(|w| ..)` method takes [`rxcrc::W`](W) writer structure
impl crate::Writable for RXCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RXCRC to value 0
impl crate::Resettable for RXCRCrs {
    const RESET_VALUE: u32 = 0;
}
