///Register `BLERXTX_ISCR` reader
pub type R = crate::R<BLERXTX_ISCRrs>;
///Register `BLERXTX_ISCR` writer
pub type W = crate::W<BLERXTX_ISCRrs>;
///Field `TX_ISC` reader - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred
pub type TX_ISC_R = crate::BitReader;
///Field `TX_ISC` writer - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred
pub type TX_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_ISC` reader - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred
pub type RX_ISC_R = crate::BitReader;
///Field `RX_ISC` writer - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred
pub type RX_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ISEDGE` reader - TX_ISEDGE: interrupt edge status on TX_SEQUENCE signal: 0: falling edge on TX_SEQUENCE detected. 1: rising edge on TX_SEQUENCE detected.
pub type TX_ISEDGE_R = crate::BitReader;
///Field `RX_ISEDGE` reader - RX_ISEDGE: interrupt edge status on RX_SEQUENCE signal: 0: falling edge on RX_SEQUENCE detected. 1: rising edge on RX_SEQUENCE detected.
pub type RX_ISEDGE_R = crate::BitReader;
impl R {
    ///Bit 0 - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred
    #[inline(always)]
    pub fn tx_isc(&self) -> TX_ISC_R {
        TX_ISC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred
    #[inline(always)]
    pub fn rx_isc(&self) -> RX_ISC_R {
        RX_ISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX_ISEDGE: interrupt edge status on TX_SEQUENCE signal: 0: falling edge on TX_SEQUENCE detected. 1: rising edge on TX_SEQUENCE detected.
    #[inline(always)]
    pub fn tx_isedge(&self) -> TX_ISEDGE_R {
        TX_ISEDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX_ISEDGE: interrupt edge status on RX_SEQUENCE signal: 0: falling edge on RX_SEQUENCE detected. 1: rising edge on RX_SEQUENCE detected.
    #[inline(always)]
    pub fn rx_isedge(&self) -> RX_ISEDGE_R {
        RX_ISEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERXTX_ISCR")
            .field("tx_isc", &self.tx_isc())
            .field("rx_isc", &self.rx_isc())
            .field("tx_isedge", &self.tx_isedge())
            .field("rx_isedge", &self.rx_isedge())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred
    #[inline(always)]
    pub fn tx_isc(&mut self) -> TX_ISC_W<'_, BLERXTX_ISCRrs> {
        TX_ISC_W::new(self, 0)
    }
    ///Bit 1 - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred
    #[inline(always)]
    pub fn rx_isc(&mut self) -> RX_ISC_W<'_, BLERXTX_ISCRrs> {
        RX_ISC_W::new(self, 1)
    }
}
/**BLERXTX_ISCR register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_iscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_iscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_ISCR)*/
pub struct BLERXTX_ISCRrs;
impl crate::RegisterSpec for BLERXTX_ISCRrs {
    type Ux = u8;
}
///`read()` method returns [`blerxtx_iscr::R`](R) reader structure
impl crate::Readable for BLERXTX_ISCRrs {}
///`write(|w| ..)` method takes [`blerxtx_iscr::W`](W) writer structure
impl crate::Writable for BLERXTX_ISCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLERXTX_ISCR to value 0
impl crate::Resettable for BLERXTX_ISCRrs {}
