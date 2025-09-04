///Register `DMAMR` reader
pub type R = crate::R<DMAMRrs>;
///Register `DMAMR` writer
pub type W = crate::W<DMAMRrs>;
///Field `SWR` reader - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock.
pub type SWR_R = crate::BitReader;
///Field `SWR` writer - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock.
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\[14:12\] and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path.
pub type DA_R = crate::BitReader;
///Field `DA` writer - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\[14:12\] and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path.
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPR` reader - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus.
pub type TXPR_R = crate::BitReader;
///Field `TXPR` writer - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus.
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR` reader - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set.
pub type PR_R = crate::FieldReader;
///Field `PR` writer - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set.
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `INTM` reader - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table 535: Transfer complete interrupt behavior).
pub type INTM_R = crate::FieldReader;
///Field `INTM` writer - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table 535: Transfer complete interrupt behavior).
pub type INTM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock.
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\[14:12\] and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path.
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 11 - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus.
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set.
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:17 - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table 535: Transfer complete interrupt behavior).
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMR")
            .field("swr", &self.swr())
            .field("da", &self.da())
            .field("txpr", &self.txpr())
            .field("pr", &self.pr())
            .field("intm", &self.intm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock.
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<DMAMRrs> {
        SWR_W::new(self, 0)
    }
    ///Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\[14:12\] and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path.
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<DMAMRrs> {
        DA_W::new(self, 1)
    }
    ///Bit 11 - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus.
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<DMAMRrs> {
        TXPR_W::new(self, 11)
    }
    ///Bits 12:14 - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set.
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<DMAMRrs> {
        PR_W::new(self, 12)
    }
    ///Bits 16:17 - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table 535: Transfer complete interrupt behavior).
    #[inline(always)]
    pub fn intm(&mut self) -> INTM_W<DMAMRrs> {
        INTM_W::new(self, 16)
    }
}
/**DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMAMR)*/
pub struct DMAMRrs;
impl crate::RegisterSpec for DMAMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamr::R`](R) reader structure
impl crate::Readable for DMAMRrs {}
///`write(|w| ..)` method takes [`dmamr::W`](W) writer structure
impl crate::Writable for DMAMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMR to value 0
impl crate::Resettable for DMAMRrs {}
