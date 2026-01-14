///Register `BLERXTX_IER` reader
pub type R = crate::R<BLERXTX_IERrs>;
///Register `BLERXTX_IER` writer
pub type W = crate::W<BLERXTX_IERrs>;
///Field `TX_IE` reader - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
pub type TX_IE_R = crate::BitReader;
///Field `TX_IE` writer - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
pub type TX_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IE` reader - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
pub type RX_IE_R = crate::BitReader;
///Field `RX_IE` writer - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
pub type RX_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn tx_ie(&self) -> TX_IE_R {
        TX_IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn rx_ie(&self) -> RX_IE_R {
        RX_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERXTX_IER")
            .field("tx_ie", &self.tx_ie())
            .field("rx_ie", &self.rx_ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn tx_ie(&mut self) -> TX_IE_W<'_, BLERXTX_IERrs> {
        TX_IE_W::new(self, 0)
    }
    ///Bit 1 - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn rx_ie(&mut self) -> RX_IE_W<'_, BLERXTX_IERrs> {
        RX_IE_W::new(self, 1)
    }
}
/**BLERXTX_IER register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_IER)*/
pub struct BLERXTX_IERrs;
impl crate::RegisterSpec for BLERXTX_IERrs {
    type Ux = u32;
}
///`read()` method returns [`blerxtx_ier::R`](R) reader structure
impl crate::Readable for BLERXTX_IERrs {}
///`write(|w| ..)` method takes [`blerxtx_ier::W`](W) writer structure
impl crate::Writable for BLERXTX_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLERXTX_IER to value 0
impl crate::Resettable for BLERXTX_IERrs {}
