///Register `MTLTXQOMR` reader
pub type R = crate::R<MTLTXQOMRrs>;
///Register `MTLTXQOMR` writer
pub type W = crate::W<MTLTXQOMRrs>;
///Field `FTQ` reader - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active.
pub type FTQ_R = crate::BitReader;
///Field `FTQ` writer - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active.
pub type FTQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSF` reader - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\[6:4\] of this register are ignored. This bit should be changed only when the transmission is stopped.
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\[6:4\] of this register are ignored. This bit should be changed only when the transmission is stopped.
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXQEN` reader - Transmit Queue Enable This field is used to enable/disable the transmit queue . Others: Reserved, must not be used. Note: In multiple Tx queues configuration, all the queues are disabled by default. Enable the Tx queue by programming this field.
pub type TXQEN_R = crate::FieldReader;
///Field `TTC` reader - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset.
pub type TTC_R = crate::FieldReader;
///Field `TTC` writer - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset.
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TQS` reader - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111).
pub type TQS_R = crate::FieldReader;
///Field `TQS` writer - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111).
pub type TQS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active.
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\[6:4\] of this register are ignored. This bit should be changed only when the transmission is stopped.
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue . Others: Reserved, must not be used. Note: In multiple Tx queues configuration, all the queues are disabled by default. Enable the Tx queue by programming this field.
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset.
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 16:18 - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111).
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQOMR")
            .field("ftq", &self.ftq())
            .field("tsf", &self.tsf())
            .field("txqen", &self.txqen())
            .field("ttc", &self.ttc())
            .field("tqs", &self.tqs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active.
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W<MTLTXQOMRrs> {
        FTQ_W::new(self, 0)
    }
    ///Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\[6:4\] of this register are ignored. This bit should be changed only when the transmission is stopped.
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<MTLTXQOMRrs> {
        TSF_W::new(self, 1)
    }
    ///Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset.
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<MTLTXQOMRrs> {
        TTC_W::new(self, 4)
    }
    ///Bits 16:18 - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111).
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W<MTLTXQOMRrs> {
        TQS_W::new(self, 16)
    }
}
/**Tx queue operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltxqomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxqomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MTLTXQOMR)*/
pub struct MTLTXQOMRrs;
impl crate::RegisterSpec for MTLTXQOMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxqomr::R`](R) reader structure
impl crate::Readable for MTLTXQOMRrs {}
///`write(|w| ..)` method takes [`mtltxqomr::W`](W) writer structure
impl crate::Writable for MTLTXQOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQOMR to value 0x0007_0008
impl crate::Resettable for MTLTXQOMRrs {
    const RESET_VALUE: u32 = 0x0007_0008;
}
