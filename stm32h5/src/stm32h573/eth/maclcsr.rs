#[doc = "Register `MACLCSR` reader"]
pub type R = crate::R<MACLCSRrs>;
#[doc = "Register `MACLCSR` writer"]
pub type W = crate::W<MACLCSRrs>;
#[doc = "Field `TLPIEN` reader - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
pub type TLPIEN_R = crate::BitReader;
#[doc = "Field `TLPIEX` reader - Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
pub type TLPIEX_R = crate::BitReader;
#[doc = "Field `RLPIEN` reader - Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set). Note: This bit may not be set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than three clock cycles of CSR clock."]
pub type RLPIEN_R = crate::BitReader;
#[doc = "Field `RLPIEX` reader - Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set). Note: This bit may not be set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than three clock cycles of CSR clock."]
pub type RLPIEX_R = crate::BitReader;
#[doc = "Field `TLPIST` reader - Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the MII interface."]
pub type TLPIST_R = crate::BitReader;
#[doc = "Field `RLPIST` reader - Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the MII interface."]
pub type RLPIST_R = crate::BitReader;
#[doc = "Field `LPIEN` reader - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state. When this bit is reset, it instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
pub type LPIEN_R = crate::BitReader;
#[doc = "Field `LPIEN` writer - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state. When this bit is reset, it instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
pub type LPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (OKAY) at least for the time indicated by the LPI LS TIMER. When this bit is set, the link is considered to be okay (UP) and when this bit is reset, the link is considered to be down."]
pub type PLS_R = crate::BitReader;
#[doc = "Field `PLS` writer - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (OKAY) at least for the time indicated by the LPI LS TIMER. When this bit is set, the link is considered to be okay (UP) and when this bit is reset, the link is considered to be down."]
pub type PLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITXA` reader - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding packets (in the core) and pending packets (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any packet for transmission or the application issues a Tx FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If Tx FIFO Flush is set in the FTQ bit of ETH_MTLTxQOMR, when the MAC is in the LPI mode, it exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
pub type LPITXA_R = crate::BitReader;
#[doc = "Field `LPITXA` writer - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding packets (in the core) and pending packets (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any packet for transmission or the application issues a Tx FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If Tx FIFO Flush is set in the FTQ bit of ETH_MTLTxQOMR, when the MAC is in the LPI mode, it exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
pub type LPITXA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITE` reader - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state. When LPITE, LPITXA and LPIEN bits are set, the MAC Transmitter enters LPI state only when the complete MAC TX data path is IDLE for a period indicated by the ETH_MACLETR register. After entering LPI state, if the data path becomes non-IDLE (due to a new packet being accepted for transmission), the Transmitter exits LPI state but does not clear LPIEN bit. This enables the re-entry into LPI state when it is IDLE again. When LPITE is 0, the LPI Auto timer is disabled and MAC Transmitter enters LPI state based on the settings of LPITXA and LPIEN bit descriptions."]
pub type LPITE_R = crate::BitReader;
#[doc = "Field `LPITE` writer - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state. When LPITE, LPITXA and LPIEN bits are set, the MAC Transmitter enters LPI state only when the complete MAC TX data path is IDLE for a period indicated by the ETH_MACLETR register. After entering LPI state, if the data path becomes non-IDLE (due to a new packet being accepted for transmission), the Transmitter exits LPI state but does not clear LPIEN bit. This enables the re-entry into LPI state when it is IDLE again. When LPITE is 0, the LPI Auto timer is disabled and MAC Transmitter enters LPI state based on the settings of LPITXA and LPIEN bit descriptions."]
pub type LPITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITCSE` reader - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped. When this bit is reset, the MAC does not assert sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode. If RGMII Interface is selected, the Tx clock is required for transmitting the LPI pattern. The Tx Clock cannot be gated and so the LPITCSE bit cannot be programmed."]
pub type LPITCSE_R = crate::BitReader;
#[doc = "Field `LPITCSE` writer - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped. When this bit is reset, the MAC does not assert sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode. If RGMII Interface is selected, the Tx clock is required for transmitting the LPI pattern. The Tx Clock cannot be gated and so the LPITCSE bit cannot be programmed."]
pub type LPITCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set). Note: This bit may not be set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than three clock cycles of CSR clock."]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register (or by writing it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set). Note: This bit may not be set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than three clock cycles of CSR clock."]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state. When this bit is reset, it instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (OKAY) at least for the time indicated by the LPI LS TIMER. When this bit is set, the link is considered to be okay (UP) and when this bit is reset, the link is considered to be down."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding packets (in the core) and pending packets (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any packet for transmission or the application issues a Tx FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If Tx FIFO Flush is set in the FTQ bit of ETH_MTLTxQOMR, when the MAC is in the LPI mode, it exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state. When LPITE, LPITXA and LPIEN bits are set, the MAC Transmitter enters LPI state only when the complete MAC TX data path is IDLE for a period indicated by the ETH_MACLETR register. After entering LPI state, if the data path becomes non-IDLE (due to a new packet being accepted for transmission), the Transmitter exits LPI state but does not clear LPIEN bit. This enables the re-entry into LPI state when it is IDLE again. When LPITE is 0, the LPI Auto timer is disabled and MAC Transmitter enters LPI state based on the settings of LPITXA and LPIEN bit descriptions."]
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped. When this bit is reset, the MAC does not assert sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode. If RGMII Interface is selected, the Tx clock is required for transmitting the LPI pattern. The Tx Clock cannot be gated and so the LPITCSE bit cannot be programmed."]
    #[inline(always)]
    pub fn lpitcse(&self) -> LPITCSE_R {
        LPITCSE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state. When this bit is reset, it instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LPIEN_W<MACLCSRrs> {
        LPIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (OKAY) at least for the time indicated by the LPI LS TIMER. When this bit is set, the link is considered to be okay (UP) and when this bit is reset, the link is considered to be down."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<MACLCSRrs> {
        PLS_W::new(self, 17)
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding packets (in the core) and pending packets (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any packet for transmission or the application issues a Tx FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If Tx FIFO Flush is set in the FTQ bit of ETH_MTLTxQOMR, when the MAC is in the LPI mode, it exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LPITXA_W<MACLCSRrs> {
        LPITXA_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state. When LPITE, LPITXA and LPIEN bits are set, the MAC Transmitter enters LPI state only when the complete MAC TX data path is IDLE for a period indicated by the ETH_MACLETR register. After entering LPI state, if the data path becomes non-IDLE (due to a new packet being accepted for transmission), the Transmitter exits LPI state but does not clear LPIEN bit. This enables the re-entry into LPI state when it is IDLE again. When LPITE is 0, the LPI Auto timer is disabled and MAC Transmitter enters LPI state based on the settings of LPITXA and LPIEN bit descriptions."]
    #[inline(always)]
    #[must_use]
    pub fn lpite(&mut self) -> LPITE_W<MACLCSRrs> {
        LPITE_W::new(self, 20)
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped. When this bit is reset, the MAC does not assert sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode. If RGMII Interface is selected, the Tx clock is required for transmitting the LPI pattern. The Tx Clock cannot be gated and so the LPITCSE bit cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn lpitcse(&mut self) -> LPITCSE_W<MACLCSRrs> {
        LPITCSE_W::new(self, 21)
    }
}
#[doc = "LPI control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maclcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maclcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLCSRrs;
impl crate::RegisterSpec for MACLCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maclcsr::R`](R) reader structure"]
impl crate::Readable for MACLCSRrs {}
#[doc = "`write(|w| ..)` method takes [`maclcsr::W`](W) writer structure"]
impl crate::Writable for MACLCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACLCSR to value 0"]
impl crate::Resettable for MACLCSRrs {
    const RESET_VALUE: u32 = 0;
}
