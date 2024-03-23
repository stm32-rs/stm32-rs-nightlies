#[doc = "Register `FDCAN_ILS` reader"]
pub type R = crate::R<FDCAN_ILSrs>;
#[doc = "Register `FDCAN_ILS` writer"]
pub type W = crate::W<FDCAN_ILSrs>;
#[doc = "Field `RxFIFO0` reader - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
pub type RX_FIFO0_R = crate::BitReader;
#[doc = "Field `RxFIFO0` writer - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
pub type RX_FIFO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFIFO1` reader - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full Interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
pub type RX_FIFO1_R = crate::BitReader;
#[doc = "Field `RxFIFO1` writer - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full Interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
pub type RX_FIFO1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSG` reader - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
pub type SMSG_R = crate::BitReader;
#[doc = "Field `SMSG` writer - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
pub type SMSG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFERR` reader - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
pub type TFERR_R = crate::BitReader;
#[doc = "Field `TFERR` writer - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
pub type TFERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC` reader - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
pub type MISC_R = crate::BitReader;
#[doc = "Field `MISC` writer - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
pub type MISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `PERR` writer - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
    #[inline(always)]
    pub fn rx_fifo0(&self) -> RX_FIFO0_R {
        RX_FIFO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full Interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
    #[inline(always)]
    pub fn rx_fifo1(&self) -> RX_FIFO1_R {
        RX_FIFO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
    #[inline(always)]
    pub fn smsg(&self) -> SMSG_R {
        SMSG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
    #[inline(always)]
    pub fn tferr(&self) -> TFERR_R {
        TFERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo0(&mut self) -> RX_FIFO0_W<FDCAN_ILSrs> {
        RX_FIFO0_W::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full Interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo1(&mut self) -> RX_FIFO1_W<FDCAN_ILSrs> {
        RX_FIFO1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn smsg(&mut self) -> SMSG_W<FDCAN_ILSrs> {
        SMSG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn tferr(&mut self) -> TFERR_W<FDCAN_ILSrs> {
        TFERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn misc(&mut self) -> MISC_W<FDCAN_ILSrs> {
        MISC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<FDCAN_ILSrs> {
        BERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<FDCAN_ILSrs> {
        PERR_W::new(self, 6)
    }
}
#[doc = "FDCAN interrupt line select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ils::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ils::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_ILSrs;
impl crate::RegisterSpec for FDCAN_ILSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ils::R`](R) reader structure"]
impl crate::Readable for FDCAN_ILSrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ils::W`](W) writer structure"]
impl crate::Writable for FDCAN_ILSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_ILS to value 0"]
impl crate::Resettable for FDCAN_ILSrs {
    const RESET_VALUE: u32 = 0;
}
