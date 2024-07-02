///Register `UCPD_TXDR` reader
pub type R = crate::R<UCPD_TXDRrs>;
///Register `UCPD_TXDR` writer
pub type W = crate::W<UCPD_TXDRrs>;
///Field `TXDATA` reader - Data byte to transmit
pub type TXDATA_R = crate::FieldReader;
///Field `TXDATA` writer - Data byte to transmit
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data byte to transmit
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPD_TXDR")
            .field("txdata", &self.txdata())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data byte to transmit
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<UCPD_TXDRrs> {
        TXDATA_W::new(self, 0)
    }
}
/**UCPD Tx data register

You can [`read`](crate::Reg::read) this register and get [`ucpd_txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_TXDR)*/
pub struct UCPD_TXDRrs;
impl crate::RegisterSpec for UCPD_TXDRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpd_txdr::R`](R) reader structure
impl crate::Readable for UCPD_TXDRrs {}
///`write(|w| ..)` method takes [`ucpd_txdr::W`](W) writer structure
impl crate::Writable for UCPD_TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets UCPD_TXDR to value 0
impl crate::Resettable for UCPD_TXDRrs {
    const RESET_VALUE: u32 = 0;
}
