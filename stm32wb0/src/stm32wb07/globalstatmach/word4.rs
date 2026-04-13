///Register `WORD4` reader
pub type R = crate::R<WORD4rs>;
///Register `WORD4` writer
pub type W = crate::W<WORD4rs>;
///Field `TxReadyTimeout` reader - Transmission ready timeout.
pub type TX_READY_TIMEOUT_R = crate::FieldReader;
///Field `TxReadyTimeout` writer - Transmission ready timeout.
pub type TX_READY_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RcvTimeout` reader - Receive window timeout.
pub type RCV_TIMEOUT_R = crate::FieldReader<u32>;
///Field `RcvTimeout` writer - Receive window timeout.
pub type RCV_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:7 - Transmission ready timeout.
    #[inline(always)]
    pub fn tx_ready_timeout(&self) -> TX_READY_TIMEOUT_R {
        TX_READY_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:27 - Receive window timeout.
    #[inline(always)]
    pub fn rcv_timeout(&self) -> RCV_TIMEOUT_R {
        RCV_TIMEOUT_R::new((self.bits >> 8) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD4")
            .field("tx_ready_timeout", &self.tx_ready_timeout())
            .field("rcv_timeout", &self.rcv_timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Transmission ready timeout.
    #[inline(always)]
    pub fn tx_ready_timeout(&mut self) -> TX_READY_TIMEOUT_W<'_, WORD4rs> {
        TX_READY_TIMEOUT_W::new(self, 0)
    }
    ///Bits 8:27 - Receive window timeout.
    #[inline(always)]
    pub fn rcv_timeout(&mut self) -> RCV_TIMEOUT_W<'_, WORD4rs> {
        RCV_TIMEOUT_W::new(self, 8)
    }
}
/**WORD4 register

You can [`read`](crate::Reg::read) this register and get [`word4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#GLOBALSTATMACH:WORD4)*/
pub struct WORD4rs;
impl crate::RegisterSpec for WORD4rs {
    type Ux = u32;
}
///`read()` method returns [`word4::R`](R) reader structure
impl crate::Readable for WORD4rs {}
///`write(|w| ..)` method takes [`word4::W`](W) writer structure
impl crate::Writable for WORD4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD4 to value 0
impl crate::Resettable for WORD4rs {}
