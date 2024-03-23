#[doc = "Register `DMACTXCR` reader"]
pub type R = crate::R<DMACTXCRrs>;
#[doc = "Register `DMACTXCR` writer"]
pub type W = crate::W<DMACTXCRrs>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state."]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state."]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\\[19\\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4."]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\\[19\\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4."]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPBL` reader - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\\[5:0\\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
pub type TXPBL_R = crate::FieldReader;
#[doc = "Field `TXPBL` writer - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\\[5:0\\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
pub type TXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\\[19\\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4."]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\\[5:0\\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<DMACTXCRrs> {
        ST_W::new(self, 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMACTXCRrs> {
        OSF_W::new(self, 4)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\\[19\\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4."]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<DMACTXCRrs> {
        TSE_W::new(self, 12)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\\[5:0\\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
    #[inline(always)]
    #[must_use]
    pub fn txpbl(&mut self) -> TXPBL_W<DMACTXCRrs> {
        TXPBL_W::new(self, 16)
    }
}
#[doc = "Channel transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactxcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactxcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTXCRrs;
impl crate::RegisterSpec for DMACTXCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactxcr::R`](R) reader structure"]
impl crate::Readable for DMACTXCRrs {}
#[doc = "`write(|w| ..)` method takes [`dmactxcr::W`](W) writer structure"]
impl crate::Writable for DMACTXCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTXCR to value 0"]
impl crate::Resettable for DMACTXCRrs {
    const RESET_VALUE: u32 = 0;
}
