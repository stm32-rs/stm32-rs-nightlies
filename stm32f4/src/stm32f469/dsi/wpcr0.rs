///Register `WPCR0` reader
pub type R = crate::R<WPCR0rs>;
///Register `WPCR0` writer
pub type W = crate::W<WPCR0rs>;
///Field `UIX4` reader - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
pub type UIX4_R = crate::FieldReader;
///Field `UIX4` writer - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
pub type UIX4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
///Field `HSICL` reader - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
pub type HSICL_R = crate::BitReader;
///Field `HSICL` writer - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
pub type HSICL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIDL0` reader - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
pub type HSIDL0_R = crate::BitReader;
///Field `HSIDL0` writer - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
pub type HSIDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIDL1` reader - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
pub type HSIDL1_R = crate::BitReader;
///Field `HSIDL1` writer - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
pub type HSIDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMCL` reader - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMCL_R = crate::BitReader;
///Field `FTXSMCL` writer - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMDL` reader - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMDL_R = crate::BitReader;
///Field `FTXSMDL` writer - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDOFFDL` reader - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
pub type CDOFFDL_R = crate::BitReader;
///Field `CDOFFDL` writer - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
pub type CDOFFDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDDL` reader - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
pub type TDDL_R = crate::BitReader;
///Field `TDDL` writer - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
pub type TDDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDEN` reader - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
pub type PDEN_R = crate::BitReader;
///Field `PDEN` writer - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLKPREPEN` reader - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
pub type TCLKPREPEN_R = crate::BitReader;
///Field `TCLKPREPEN` writer - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
pub type TCLKPREPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLKZEROEN` reader - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
pub type TCLKZEROEN_R = crate::BitReader;
///Field `TCLKZEROEN` writer - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
pub type TCLKZEROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSPREPEN` reader - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
pub type THSPREPEN_R = crate::BitReader;
///Field `THSPREPEN` writer - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
pub type THSPREPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSTRAILEN` reader - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
pub type THSTRAILEN_R = crate::BitReader;
///Field `THSTRAILEN` writer - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
pub type THSTRAILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSZEROEN` reader - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
pub type THSZEROEN_R = crate::BitReader;
///Field `THSZEROEN` writer - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
pub type THSZEROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TLPXDEN` reader - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
pub type TLPXDEN_R = crate::BitReader;
///Field `TLPXDEN` writer - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
pub type TLPXDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSEXITEN` reader - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
pub type THSEXITEN_R = crate::BitReader;
///Field `THSEXITEN` writer - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
pub type THSEXITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TLPXCEN` reader - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
pub type TLPXCEN_R = crate::BitReader;
///Field `TLPXCEN` writer - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
pub type TLPXCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLKPOSTEN` reader - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
pub type TCLKPOSTEN_R = crate::BitReader;
///Field `TCLKPOSTEN` writer - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
pub type TCLKPOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
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
    ///Bit 9 - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
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
    ///Bit 14 - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR0")
            .field("uix4", &self.uix4())
            .field("swcl", &self.swcl())
            .field("swdl0", &self.swdl0())
            .field("swdl1", &self.swdl1())
            .field("hsicl", &self.hsicl())
            .field("hsidl0", &self.hsidl0())
            .field("hsidl1", &self.hsidl1())
            .field("ftxsmcl", &self.ftxsmcl())
            .field("ftxsmdl", &self.ftxsmdl())
            .field("cdoffdl", &self.cdoffdl())
            .field("tddl", &self.tddl())
            .field("pden", &self.pden())
            .field("tclkprepen", &self.tclkprepen())
            .field("tclkzeroen", &self.tclkzeroen())
            .field("thsprepen", &self.thsprepen())
            .field("thstrailen", &self.thstrailen())
            .field("thszeroen", &self.thszeroen())
            .field("tlpxden", &self.tlpxden())
            .field("thsexiten", &self.thsexiten())
            .field("tlpxcen", &self.tlpxcen())
            .field("tclkposten", &self.tclkposten())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W<WPCR0rs> {
        UIX4_W::new(self, 0)
    }
    ///Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W<WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    ///Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0.
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W<WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    ///Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W<WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    ///Bit 9 - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W<WPCR0rs> {
        HSICL_W::new(self, 9)
    }
    ///Bit 10 - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W<WPCR0rs> {
        HSIDL0_W::new(self, 10)
    }
    ///Bit 11 - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W<WPCR0rs> {
        HSIDL1_W::new(self, 11)
    }
    ///Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    ///Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
    ///Bit 14 - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<WPCR0rs> {
        CDOFFDL_W::new(self, 14)
    }
    ///Bit 16 - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W<WPCR0rs> {
        TDDL_W::new(self, 16)
    }
    ///Bit 18 - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<WPCR0rs> {
        PDEN_W::new(self, 18)
    }
    ///Bit 19 - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<WPCR0rs> {
        TCLKPREPEN_W::new(self, 19)
    }
    ///Bit 20 - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<WPCR0rs> {
        TCLKZEROEN_W::new(self, 20)
    }
    ///Bit 21 - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<WPCR0rs> {
        THSPREPEN_W::new(self, 21)
    }
    ///Bit 22 - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<WPCR0rs> {
        THSTRAILEN_W::new(self, 22)
    }
    ///Bit 23 - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<WPCR0rs> {
        THSZEROEN_W::new(self, 23)
    }
    ///Bit 24 - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<WPCR0rs> {
        TLPXDEN_W::new(self, 24)
    }
    ///Bit 25 - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<WPCR0rs> {
        THSEXITEN_W::new(self, 25)
    }
    ///Bit 26 - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<WPCR0rs> {
        TLPXCEN_W::new(self, 26)
    }
    ///Bit 27 - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
    #[inline(always)]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<WPCR0rs> {
        TCLKPOSTEN_W::new(self, 27)
    }
}
/**DSI Wrapper PHY configuration register 0

You can [`read`](crate::Reg::read) this register and get [`wpcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WPCR0)*/
pub struct WPCR0rs;
impl crate::RegisterSpec for WPCR0rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr0::R`](R) reader structure
impl crate::Readable for WPCR0rs {}
///`write(|w| ..)` method takes [`wpcr0::W`](W) writer structure
impl crate::Writable for WPCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR0 to value 0
impl crate::Resettable for WPCR0rs {}
