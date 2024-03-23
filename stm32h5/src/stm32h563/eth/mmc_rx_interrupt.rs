#[doc = "Register `MMC_RX_INTERRUPT` reader"]
pub type R = crate::R<MMC_RX_INTERRUPTrs>;
#[doc = "Register `MMC_RX_INTERRUPT` writer"]
pub type W = crate::W<MMC_RX_INTERRUPTrs>;
#[doc = "Field `RXCRCERPIS` reader - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXCRCERPIS_R = crate::BitReader;
#[doc = "Field `RXCRCERPIS` writer - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
pub type RXCRCERPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXALGNERPIS` reader - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXALGNERPIS_R = crate::BitReader;
#[doc = "Field `RXALGNERPIS` writer - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
pub type RXALGNERPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUCGPIS` reader - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXUCGPIS_R = crate::BitReader;
#[doc = "Field `RXUCGPIS` writer - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value."]
pub type RXUCGPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIUSCIS` reader - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXLPIUSCIS_R = crate::BitReader;
#[doc = "Field `RXLPIUSCIS` writer - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
pub type RXLPIUSCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPITRCIS` reader - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXLPITRCIS_R = crate::BitReader;
#[doc = "Field `RXLPITRCIS` writer - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
pub type RXLPITRCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerpis(&mut self) -> RXCRCERPIS_W<MMC_RX_INTERRUPTrs> {
        RXCRCERPIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxalgnerpis(&mut self) -> RXALGNERPIS_W<MMC_RX_INTERRUPTrs> {
        RXALGNERPIS_W::new(self, 6)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxucgpis(&mut self) -> RXUCGPIS_W<MMC_RX_INTERRUPTrs> {
        RXUCGPIS_W::new(self, 17)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiuscis(&mut self) -> RXLPIUSCIS_W<MMC_RX_INTERRUPTrs> {
        RXLPIUSCIS_W::new(self, 26)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxlpitrcis(&mut self) -> RXLPITRCIS_W<MMC_RX_INTERRUPTrs> {
        RXLPITRCIS_W::new(self, 27)
    }
}
#[doc = "MMC Rx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_interrupt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_RX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_RX_INTERRUPTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPTrs {}
#[doc = "`write(|w| ..)` method takes [`mmc_rx_interrupt::W`](W) writer structure"]
impl crate::Writable for MMC_RX_INTERRUPTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPTrs {
    const RESET_VALUE: u32 = 0;
}
