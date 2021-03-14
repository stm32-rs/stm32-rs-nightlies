#[doc = "Reader of register DDRCTRL_PSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_PSTAT>;
#[doc = "Reader of field `RD_PORT_BUSY_0`"]
pub type RD_PORT_BUSY_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_PORT_BUSY_1`"]
pub type RD_PORT_BUSY_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_PORT_BUSY_0`"]
pub type WR_PORT_BUSY_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_PORT_BUSY_1`"]
pub type WR_PORT_BUSY_1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RD_PORT_BUSY_0"]
    #[inline(always)]
    pub fn rd_port_busy_0(&self) -> RD_PORT_BUSY_0_R {
        RD_PORT_BUSY_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RD_PORT_BUSY_1"]
    #[inline(always)]
    pub fn rd_port_busy_1(&self) -> RD_PORT_BUSY_1_R {
        RD_PORT_BUSY_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WR_PORT_BUSY_0"]
    #[inline(always)]
    pub fn wr_port_busy_0(&self) -> WR_PORT_BUSY_0_R {
        WR_PORT_BUSY_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WR_PORT_BUSY_1"]
    #[inline(always)]
    pub fn wr_port_busy_1(&self) -> WR_PORT_BUSY_1_R {
        WR_PORT_BUSY_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
