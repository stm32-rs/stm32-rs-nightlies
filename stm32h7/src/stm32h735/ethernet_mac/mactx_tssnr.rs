///Register `MACTxTSSNR` reader
pub type R = crate::R<MACTX_TSSNRrs>;
///Field `TXTSSLO` reader - Transmit Timestamp Status Low
pub type TXTSSLO_R = crate::FieldReader<u32>;
///Field `TXTSSMIS` reader - Transmit Timestamp Status Missed
pub type TXTSSMIS_R = crate::BitReader;
impl R {
    ///Bits 0:30 - Transmit Timestamp Status Low
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Transmit Timestamp Status Missed
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTxTSSNR")
            .field("txtsslo", &self.txtsslo())
            .field("txtssmis", &self.txtssmis())
            .finish()
    }
}
/**Tx timestamp status nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`mactx_tssnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#Ethernet_MAC:MACTxTSSNR)*/
pub struct MACTX_TSSNRrs;
impl crate::RegisterSpec for MACTX_TSSNRrs {
    type Ux = u32;
}
///`read()` method returns [`mactx_tssnr::R`](R) reader structure
impl crate::Readable for MACTX_TSSNRrs {}
///`reset()` method sets MACTxTSSNR to value 0
impl crate::Resettable for MACTX_TSSNRrs {}
