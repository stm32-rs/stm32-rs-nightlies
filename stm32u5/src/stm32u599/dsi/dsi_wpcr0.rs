///Register `DSI_WPCR0` reader
pub type R = crate::R<DSI_WPCR0rs>;
///Register `DSI_WPCR0` writer
pub type W = crate::W<DSI_WPCR0rs>;
///Field `SWCL` reader - Swap clock lane pins This bit swaps the pins on clock lane.
pub type SWCL_R = crate::BitReader;
///Field `SWCL` writer - Swap clock lane pins This bit swaps the pins on clock lane.
pub type SWCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWDL0` reader - Swap data lane 0 pins This bit swaps the pins on data lane 0.
pub type SWDL0_R = crate::BitReader;
///Field `SWDL0` writer - Swap data lane 0 pins This bit swaps the pins on data lane 0.
pub type SWDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWDL1` reader - Swap data lane 1 pins This bit swaps the pins on clock lane.
pub type SWDL1_R = crate::BitReader;
///Field `SWDL1` writer - Swap data lane 1 pins This bit swaps the pins on clock lane.
pub type SWDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMCL` reader - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMCL_R = crate::BitReader;
///Field `FTXSMCL` writer - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMDL` reader - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMDL_R = crate::BitReader;
///Field `FTXSMDL` writer - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0.
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_WPCR0")
            .field("swcl", &self.swcl())
            .field("swdl0", &self.swdl0())
            .field("swdl1", &self.swdl1())
            .field("ftxsmcl", &self.ftxsmcl())
            .field("ftxsmdl", &self.ftxsmdl())
            .finish()
    }
}
impl W {
    ///Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane.
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<DSI_WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    ///Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0.
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<DSI_WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    ///Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane.
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<DSI_WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    ///Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<DSI_WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    ///Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<DSI_WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
}
/**DSI Wrapper PHY configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dsi_wpcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_WPCR0)*/
pub struct DSI_WPCR0rs;
impl crate::RegisterSpec for DSI_WPCR0rs {
    type Ux = u32;
}
///`read()` method returns [`dsi_wpcr0::R`](R) reader structure
impl crate::Readable for DSI_WPCR0rs {}
///`write(|w| ..)` method takes [`dsi_wpcr0::W`](W) writer structure
impl crate::Writable for DSI_WPCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_WPCR0 to value 0
impl crate::Resettable for DSI_WPCR0rs {
    const RESET_VALUE: u32 = 0;
}
