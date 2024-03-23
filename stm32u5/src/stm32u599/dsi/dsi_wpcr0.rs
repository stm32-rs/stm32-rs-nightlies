#[doc = "Register `DSI_WPCR0` reader"]
pub type R = crate::R<DSI_WPCR0rs>;
#[doc = "Register `DSI_WPCR0` writer"]
pub type W = crate::W<DSI_WPCR0rs>;
#[doc = "Field `SWCL` reader - Swap clock lane pins This bit swaps the pins on clock lane."]
pub type SWCL_R = crate::BitReader;
#[doc = "Field `SWCL` writer - Swap clock lane pins This bit swaps the pins on clock lane."]
pub type SWCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL0` reader - Swap data lane 0 pins This bit swaps the pins on data lane 0."]
pub type SWDL0_R = crate::BitReader;
#[doc = "Field `SWDL0` writer - Swap data lane 0 pins This bit swaps the pins on data lane 0."]
pub type SWDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL1` reader - Swap data lane 1 pins This bit swaps the pins on clock lane."]
pub type SWDL1_R = crate::BitReader;
#[doc = "Field `SWDL1` writer - Swap data lane 1 pins This bit swaps the pins on clock lane."]
pub type SWDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMCL` reader - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
pub type FTXSMCL_R = crate::BitReader;
#[doc = "Field `FTXSMCL` writer - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMDL` reader - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
pub type FTXSMDL_R = crate::BitReader;
#[doc = "Field `FTXSMDL` writer - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane."]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0."]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane."]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane."]
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<DSI_WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0."]
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<DSI_WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    #[doc = "Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane."]
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<DSI_WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    #[doc = "Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<DSI_WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<DSI_WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
}
#[doc = "DSI Wrapper PHY configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WPCR0rs;
impl crate::RegisterSpec for DSI_WPCR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr0::R`](R) reader structure"]
impl crate::Readable for DSI_WPCR0rs {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr0::W`](W) writer structure"]
impl crate::Writable for DSI_WPCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_WPCR0 to value 0"]
impl crate::Resettable for DSI_WPCR0rs {
    const RESET_VALUE: u32 = 0;
}
