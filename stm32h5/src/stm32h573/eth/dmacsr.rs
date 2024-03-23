#[doc = "Register `DMACSR` reader"]
pub type R = crate::R<DMACSRrs>;
#[doc = "Register `DMACSR` writer"]
pub type W = crate::W<DMACSRrs>;
#[doc = "Field `TI` reader - Transmit Interrupt This bit indicates that the packet transmission is complete. When transmission is complete, Bit 31 of TDES3 is reset in the last descriptor, and the specific packet status information is updated in the descriptor."]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt This bit indicates that the packet transmission is complete. When transmission is complete, Bit 31 of TDES3 is reset in the last descriptor, and the specific packet status information is updated in the descriptor."]
pub type TI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it. Transmission is suspended. The TPSi field of the Debug status register (ETH_DMADSR) register explains the Transmit Process state transitions. To resume processing the Transmit descriptors, the application should do the following: 1. Change the ownership of the descriptor by setting Bit 31 of TDES3. 2. Issue a Transmit Poll Demand command. For ring mode, the application should advance the Transmit Descriptor Tail Pointer register of a channel."]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it. Transmission is suspended. The TPSi field of the Debug status register (ETH_DMADSR) register explains the Transmit Process state transitions. To resume processing the Transmit descriptors, the application should do the following: 1. Change the ownership of the descriptor by setting Bit 31 of TDES3. 2. Issue a Transmit Poll Demand command. For ring mode, the application should advance the Transmit Descriptor Tail Pointer register of a channel."]
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt This bit indicates that the packet reception is complete. When packet reception is complete, Bit 31 of RDES1 is reset in the last descriptor, and the specific packet status information is updated in the descriptor. The reception remains in the Running state."]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt This bit indicates that the packet reception is complete. When packet reception is complete, Bit 31 of RDES1 is reset in the last descriptor, and the specific packet status information is updated in the descriptor. The reception remains in the Running state."]
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBU` reader - Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it. The Rx process is suspended. To resume processing Rx descriptors, the application should change the ownership of the descriptor and issue a Receive Poll Demand command. If this command is not issued, the Rx process resumes when the next recognized incoming packet is received. In ring mode, the application should advance the Receive Descriptor Tail Pointer register of a channel. This bit is set only when the DMA owns the previous Rx descriptor."]
pub type RBU_R = crate::BitReader;
#[doc = "Field `RBU` writer - Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it. The Rx process is suspended. To resume processing Rx descriptors, the application should change the ownership of the descriptor and issue a Receive Poll Demand command. If this command is not issued, the Rx process resumes when the next recognized incoming packet is received. In ring mode, the application should advance the Receive Descriptor Tail Pointer register of a channel. This bit is set only when the DMA owns the previous Rx descriptor."]
pub type RBU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
pub type RPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
pub type RWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
pub type ETI_R = crate::BitReader;
#[doc = "Field `ETI` writer - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
pub type ETI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERI` reader - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet.The RI bit of this register automatically clears this bit."]
pub type ERI_R = crate::BitReader;
#[doc = "Field `ERI` writer - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet.The RI bit of this register automatically clears this bit."]
pub type ERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field). When this bit is set, the corresponding DMA channel engine disables all bus accesses."]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field). When this bit is set, the corresponding DMA channel engine disables all bus accesses."]
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDE` reader - Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow (intermediate descriptor) or all one‘s descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
pub type CDE_R = crate::BitReader;
#[doc = "Field `CDE` writer - Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow (intermediate descriptor) or all one‘s descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
pub type CDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in ETH_DMACIER register) affect the Normal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in ETH_DMACIER register) affect the Normal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEB` reader - Tx DMA Error Bits This field indicates the type of error that caused a Bus Error. For example, error response on the AHB interface. Bit\\[2\\]: Error during data transfer by Tx DMA when 1, no Error during data transfer by Tx DMA when 0 Bit\\[1\\]: Error during descriptor access when 1, Error during data buffer access when 0 Bit\\[0\\]: Error during read transfer when 1, Error during write transfer when 0 This field is valid only when the FBE bit is set. This field does not generate an interrupt."]
pub type TEB_R = crate::FieldReader;
#[doc = "Field `REB` reader - Rx DMA Error Bits This field indicates the type of error that caused a Bus Error. For example, error response on the AHB interface. Bit \\[2\\]: Error during data transfer by Rx DMA when 1, no Error during data transfer by Rx DMA when 0. Bit\\[1\\]: Error during descriptor access when 1, Error during data buffer access when 0 Bit\\[0\\]: Error during read transfer when 1, Error during write transfer when 0 This field is valid only when the FBE bit is set. This field does not generate an interrupt."]
pub type REB_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the packet transmission is complete. When transmission is complete, Bit 31 of TDES3 is reset in the last descriptor, and the specific packet status information is updated in the descriptor."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it. Transmission is suspended. The TPSi field of the Debug status register (ETH_DMADSR) register explains the Transmit Process state transitions. To resume processing the Transmit descriptors, the application should do the following: 1. Change the ownership of the descriptor by setting Bit 31 of TDES3. 2. Issue a Transmit Poll Demand command. For ring mode, the application should advance the Transmit Descriptor Tail Pointer register of a channel."]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the packet reception is complete. When packet reception is complete, Bit 31 of RDES1 is reset in the last descriptor, and the specific packet status information is updated in the descriptor. The reception remains in the Running state."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it. The Rx process is suspended. To resume processing Rx descriptors, the application should change the ownership of the descriptor and issue a Receive Poll Demand command. If this command is not issued, the Rx process resumes when the next recognized incoming packet is received. In ring mode, the application should advance the Receive Descriptor Tail Pointer register of a channel. This bit is set only when the DMA owns the previous Rx descriptor."]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet.The RI bit of this register automatically clears this bit."]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field). When this bit is set, the corresponding DMA channel engine disables all bus accesses."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow (intermediate descriptor) or all one‘s descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in ETH_DMACIER register) affect the Normal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits This field indicates the type of error that caused a Bus Error. For example, error response on the AHB interface. Bit\\[2\\]: Error during data transfer by Tx DMA when 1, no Error during data transfer by Tx DMA when 0 Bit\\[1\\]: Error during descriptor access when 1, Error during data buffer access when 0 Bit\\[0\\]: Error during read transfer when 1, Error during write transfer when 0 This field is valid only when the FBE bit is set. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn teb(&self) -> TEB_R {
        TEB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits This field indicates the type of error that caused a Bus Error. For example, error response on the AHB interface. Bit \\[2\\]: Error during data transfer by Rx DMA when 1, no Error during data transfer by Rx DMA when 0. Bit\\[1\\]: Error during descriptor access when 1, Error during data buffer access when 0 Bit\\[0\\]: Error during read transfer when 1, Error during write transfer when 0 This field is valid only when the FBE bit is set. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn reb(&self) -> REB_R {
        REB_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the packet transmission is complete. When transmission is complete, Bit 31 of TDES3 is reset in the last descriptor, and the specific packet status information is updated in the descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<DMACSRrs> {
        TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<DMACSRrs> {
        TPS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it. Transmission is suspended. The TPSi field of the Debug status register (ETH_DMADSR) register explains the Transmit Process state transitions. To resume processing the Transmit descriptors, the application should do the following: 1. Change the ownership of the descriptor by setting Bit 31 of TDES3. 2. Issue a Transmit Poll Demand command. For ring mode, the application should advance the Transmit Descriptor Tail Pointer register of a channel."]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<DMACSRrs> {
        TBU_W::new(self, 2)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the packet reception is complete. When packet reception is complete, Bit 31 of RDES1 is reset in the last descriptor, and the specific packet status information is updated in the descriptor. The reception remains in the Running state."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<DMACSRrs> {
        RI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it. The Rx process is suspended. To resume processing Rx descriptors, the application should change the ownership of the descriptor and issue a Receive Poll Demand command. If this command is not issued, the Rx process resumes when the next recognized incoming packet is received. In ring mode, the application should advance the Receive Descriptor Tail Pointer register of a channel. This bit is set only when the DMA owns the previous Rx descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RBU_W<DMACSRrs> {
        RBU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<DMACSRrs> {
        RPS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMACSRrs> {
        RWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> ETI_W<DMACSRrs> {
        ETI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet.The RI bit of this register automatically clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> ERI_W<DMACSRrs> {
        ERI_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field). When this bit is set, the corresponding DMA channel engine disables all bus accesses."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<DMACSRrs> {
        FBE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow (intermediate descriptor) or all one‘s descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CDE_W<DMACSRrs> {
        CDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<DMACSRrs> {
        AIS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the ETH_DMACIER register: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in ETH_DMACIER register) affect the Normal Interrupt Summary bit. This is a sticky bit. You must clear this bit (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<DMACSRrs> {
        NIS_W::new(self, 15)
    }
}
#[doc = "Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACSRrs;
impl crate::RegisterSpec for DMACSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsr::R`](R) reader structure"]
impl crate::Readable for DMACSRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacsr::W`](W) writer structure"]
impl crate::Writable for DMACSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACSR to value 0"]
impl crate::Resettable for DMACSRrs {
    const RESET_VALUE: u32 = 0;
}
