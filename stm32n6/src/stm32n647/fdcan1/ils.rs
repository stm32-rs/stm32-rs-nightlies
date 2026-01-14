///Register `ILS` reader
pub type R = crate::R<ILSrs>;
///Register `ILS` writer
pub type W = crate::W<ILSrs>;
///Field `RF0NL` reader - Rx FIFO 0 new message interrupt line
pub type RF0NL_R = crate::BitReader;
///Field `RF0NL` writer - Rx FIFO 0 new message interrupt line
pub type RF0NL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0WL` reader - Rx FIFO 0 watermark reached interrupt line
pub type RF0WL_R = crate::BitReader;
///Field `RF0WL` writer - Rx FIFO 0 watermark reached interrupt line
pub type RF0WL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0FL` reader - Rx FIFO 0 full interrupt line
pub type RF0FL_R = crate::BitReader;
///Field `RF0FL` writer - Rx FIFO 0 full interrupt line
pub type RF0FL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0LL` reader - Rx FIFO 0 message lost interrupt line
pub type RF0LL_R = crate::BitReader;
///Field `RF0LL` writer - Rx FIFO 0 message lost interrupt line
pub type RF0LL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1NL` reader - Rx FIFO 1 new message interrupt line
pub type RF1NL_R = crate::BitReader;
///Field `RF1NL` writer - Rx FIFO 1 new message interrupt line
pub type RF1NL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1WL` reader - Rx FIFO 1 watermark reached interrupt line
pub type RF1WL_R = crate::BitReader;
///Field `RF1WL` writer - Rx FIFO 1 watermark reached interrupt line
pub type RF1WL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1FL` reader - Rx FIFO 1 full interrupt line
pub type RF1FL_R = crate::BitReader;
///Field `RF1FL` writer - Rx FIFO 1 full interrupt line
pub type RF1FL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1LL` reader - Rx FIFO 1 message lost interrupt line
pub type RF1LL_R = crate::BitReader;
///Field `RF1LL` writer - Rx FIFO 1 message lost interrupt line
pub type RF1LL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPML` reader - High priority message interrupt line
pub type HPML_R = crate::BitReader;
///Field `HPML` writer - High priority message interrupt line
pub type HPML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCL` reader - Transmission completed interrupt line
pub type TCL_R = crate::BitReader;
///Field `TCL` writer - Transmission completed interrupt line
pub type TCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCFL` reader - Transmission cancellation finished interrupt line
pub type TCFL_R = crate::BitReader;
///Field `TCFL` writer - Transmission cancellation finished interrupt line
pub type TCFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFEL` reader - Tx FIFO empty interrupt line
pub type TFEL_R = crate::BitReader;
///Field `TFEL` writer - Tx FIFO empty interrupt line
pub type TFEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFNL` reader - Tx event FIFO new entry interrupt line
pub type TEFNL_R = crate::BitReader;
///Field `TEFNL` writer - Tx event FIFO new entry interrupt line
pub type TEFNL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFWL` reader - Tx event FIFO watermark reached interrupt line
pub type TEFWL_R = crate::BitReader;
///Field `TEFWL` writer - Tx event FIFO watermark reached interrupt line
pub type TEFWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFFL` reader - Tx event FIFO full interrupt line
pub type TEFFL_R = crate::BitReader;
///Field `TEFFL` writer - Tx event FIFO full interrupt line
pub type TEFFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFLL` reader - Tx event FIFO element Lost interrupt line
pub type TEFLL_R = crate::BitReader;
///Field `TEFLL` writer - Tx event FIFO element Lost interrupt line
pub type TEFLL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSWL` reader - Timestamp wraparound interrupt line
pub type TSWL_R = crate::BitReader;
///Field `TSWL` writer - Timestamp wraparound interrupt line
pub type TSWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAFL` reader - Message RAM access failure interrupt line
pub type MRAFL_R = crate::BitReader;
///Field `MRAFL` writer - Message RAM access failure interrupt line
pub type MRAFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOOL` reader - Timeout occurred interrupt line
pub type TOOL_R = crate::BitReader;
///Field `TOOL` writer - Timeout occurred interrupt line
pub type TOOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRXL` reader - Message stored to dedicated Rx buffer interrupt line
pub type DRXL_R = crate::BitReader;
///Field `DRXL` writer - Message stored to dedicated Rx buffer interrupt line
pub type DRXL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELOL` reader - Error logging overflow interrupt line
pub type ELOL_R = crate::BitReader;
///Field `ELOL` writer - Error logging overflow interrupt line
pub type ELOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPL` reader - Error passive interrupt line
pub type EPL_R = crate::BitReader;
///Field `EPL` writer - Error passive interrupt line
pub type EPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWL` reader - Warning status interrupt line
pub type EWL_R = crate::BitReader;
///Field `EWL` writer - Warning status interrupt line
pub type EWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOL` reader - Bus_Off status
pub type BOL_R = crate::BitReader;
///Field `BOL` writer - Bus_Off status
pub type BOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDIL` reader - Watchdog interrupt line
pub type WDIL_R = crate::BitReader;
///Field `WDIL` writer - Watchdog interrupt line
pub type WDIL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEAL` reader - Protocol error in arbitration phase line
pub type PEAL_R = crate::BitReader;
///Field `PEAL` writer - Protocol error in arbitration phase line
pub type PEAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEDL` reader - Protocol error in data phase line
pub type PEDL_R = crate::BitReader;
///Field `PEDL` writer - Protocol error in data phase line
pub type PEDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARAL` reader - Access to reserved address line
pub type ARAL_R = crate::BitReader;
///Field `ARAL` writer - Access to reserved address line
pub type ARAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx FIFO 0 new message interrupt line
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 watermark reached interrupt line
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 full interrupt line
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 message lost interrupt line
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 new message interrupt line
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 watermark reached interrupt line
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 full interrupt line
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 message lost interrupt line
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High priority message interrupt line
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission completed interrupt line
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission cancellation finished interrupt line
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO empty interrupt line
    #[inline(always)]
    pub fn tfel(&self) -> TFEL_R {
        TFEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx event FIFO new entry interrupt line
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx event FIFO watermark reached interrupt line
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx event FIFO full interrupt line
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx event FIFO element Lost interrupt line
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp wraparound interrupt line
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM access failure interrupt line
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout occurred interrupt line
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to dedicated Rx buffer interrupt line
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Error logging overflow interrupt line
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error passive interrupt line
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning status interrupt line
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off status
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog interrupt line
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol error in arbitration phase line
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol error in data phase line
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to reserved address line
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILS")
            .field("rf0nl", &self.rf0nl())
            .field("rf0wl", &self.rf0wl())
            .field("rf0fl", &self.rf0fl())
            .field("rf0ll", &self.rf0ll())
            .field("rf1nl", &self.rf1nl())
            .field("rf1wl", &self.rf1wl())
            .field("rf1fl", &self.rf1fl())
            .field("rf1ll", &self.rf1ll())
            .field("hpml", &self.hpml())
            .field("tcl", &self.tcl())
            .field("tcfl", &self.tcfl())
            .field("tfel", &self.tfel())
            .field("tefnl", &self.tefnl())
            .field("tefwl", &self.tefwl())
            .field("teffl", &self.teffl())
            .field("tefll", &self.tefll())
            .field("tswl", &self.tswl())
            .field("mrafl", &self.mrafl())
            .field("tool", &self.tool())
            .field("drxl", &self.drxl())
            .field("elol", &self.elol())
            .field("epl", &self.epl())
            .field("ewl", &self.ewl())
            .field("bol", &self.bol())
            .field("wdil", &self.wdil())
            .field("peal", &self.peal())
            .field("pedl", &self.pedl())
            .field("aral", &self.aral())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 new message interrupt line
    #[inline(always)]
    pub fn rf0nl(&mut self) -> RF0NL_W<'_, ILSrs> {
        RF0NL_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 watermark reached interrupt line
    #[inline(always)]
    pub fn rf0wl(&mut self) -> RF0WL_W<'_, ILSrs> {
        RF0WL_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 full interrupt line
    #[inline(always)]
    pub fn rf0fl(&mut self) -> RF0FL_W<'_, ILSrs> {
        RF0FL_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 0 message lost interrupt line
    #[inline(always)]
    pub fn rf0ll(&mut self) -> RF0LL_W<'_, ILSrs> {
        RF0LL_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 new message interrupt line
    #[inline(always)]
    pub fn rf1nl(&mut self) -> RF1NL_W<'_, ILSrs> {
        RF1NL_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 watermark reached interrupt line
    #[inline(always)]
    pub fn rf1wl(&mut self) -> RF1WL_W<'_, ILSrs> {
        RF1WL_W::new(self, 5)
    }
    ///Bit 6 - Rx FIFO 1 full interrupt line
    #[inline(always)]
    pub fn rf1fl(&mut self) -> RF1FL_W<'_, ILSrs> {
        RF1FL_W::new(self, 6)
    }
    ///Bit 7 - Rx FIFO 1 message lost interrupt line
    #[inline(always)]
    pub fn rf1ll(&mut self) -> RF1LL_W<'_, ILSrs> {
        RF1LL_W::new(self, 7)
    }
    ///Bit 8 - High priority message interrupt line
    #[inline(always)]
    pub fn hpml(&mut self) -> HPML_W<'_, ILSrs> {
        HPML_W::new(self, 8)
    }
    ///Bit 9 - Transmission completed interrupt line
    #[inline(always)]
    pub fn tcl(&mut self) -> TCL_W<'_, ILSrs> {
        TCL_W::new(self, 9)
    }
    ///Bit 10 - Transmission cancellation finished interrupt line
    #[inline(always)]
    pub fn tcfl(&mut self) -> TCFL_W<'_, ILSrs> {
        TCFL_W::new(self, 10)
    }
    ///Bit 11 - Tx FIFO empty interrupt line
    #[inline(always)]
    pub fn tfel(&mut self) -> TFEL_W<'_, ILSrs> {
        TFEL_W::new(self, 11)
    }
    ///Bit 12 - Tx event FIFO new entry interrupt line
    #[inline(always)]
    pub fn tefnl(&mut self) -> TEFNL_W<'_, ILSrs> {
        TEFNL_W::new(self, 12)
    }
    ///Bit 13 - Tx event FIFO watermark reached interrupt line
    #[inline(always)]
    pub fn tefwl(&mut self) -> TEFWL_W<'_, ILSrs> {
        TEFWL_W::new(self, 13)
    }
    ///Bit 14 - Tx event FIFO full interrupt line
    #[inline(always)]
    pub fn teffl(&mut self) -> TEFFL_W<'_, ILSrs> {
        TEFFL_W::new(self, 14)
    }
    ///Bit 15 - Tx event FIFO element Lost interrupt line
    #[inline(always)]
    pub fn tefll(&mut self) -> TEFLL_W<'_, ILSrs> {
        TEFLL_W::new(self, 15)
    }
    ///Bit 16 - Timestamp wraparound interrupt line
    #[inline(always)]
    pub fn tswl(&mut self) -> TSWL_W<'_, ILSrs> {
        TSWL_W::new(self, 16)
    }
    ///Bit 17 - Message RAM access failure interrupt line
    #[inline(always)]
    pub fn mrafl(&mut self) -> MRAFL_W<'_, ILSrs> {
        MRAFL_W::new(self, 17)
    }
    ///Bit 18 - Timeout occurred interrupt line
    #[inline(always)]
    pub fn tool(&mut self) -> TOOL_W<'_, ILSrs> {
        TOOL_W::new(self, 18)
    }
    ///Bit 19 - Message stored to dedicated Rx buffer interrupt line
    #[inline(always)]
    pub fn drxl(&mut self) -> DRXL_W<'_, ILSrs> {
        DRXL_W::new(self, 19)
    }
    ///Bit 22 - Error logging overflow interrupt line
    #[inline(always)]
    pub fn elol(&mut self) -> ELOL_W<'_, ILSrs> {
        ELOL_W::new(self, 22)
    }
    ///Bit 23 - Error passive interrupt line
    #[inline(always)]
    pub fn epl(&mut self) -> EPL_W<'_, ILSrs> {
        EPL_W::new(self, 23)
    }
    ///Bit 24 - Warning status interrupt line
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W<'_, ILSrs> {
        EWL_W::new(self, 24)
    }
    ///Bit 25 - Bus_Off status
    #[inline(always)]
    pub fn bol(&mut self) -> BOL_W<'_, ILSrs> {
        BOL_W::new(self, 25)
    }
    ///Bit 26 - Watchdog interrupt line
    #[inline(always)]
    pub fn wdil(&mut self) -> WDIL_W<'_, ILSrs> {
        WDIL_W::new(self, 26)
    }
    ///Bit 27 - Protocol error in arbitration phase line
    #[inline(always)]
    pub fn peal(&mut self) -> PEAL_W<'_, ILSrs> {
        PEAL_W::new(self, 27)
    }
    ///Bit 28 - Protocol error in data phase line
    #[inline(always)]
    pub fn pedl(&mut self) -> PEDL_W<'_, ILSrs> {
        PEDL_W::new(self, 28)
    }
    ///Bit 29 - Access to reserved address line
    #[inline(always)]
    pub fn aral(&mut self) -> ARAL_W<'_, ILSrs> {
        ARAL_W::new(self, 29)
    }
}
/**FDCAN interrupt line select register

You can [`read`](crate::Reg::read) this register and get [`ils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:ILS)*/
pub struct ILSrs;
impl crate::RegisterSpec for ILSrs {
    type Ux = u32;
}
///`read()` method returns [`ils::R`](R) reader structure
impl crate::Readable for ILSrs {}
///`write(|w| ..)` method takes [`ils::W`](W) writer structure
impl crate::Writable for ILSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ILS to value 0
impl crate::Resettable for ILSrs {}
