#[doc = "Register `DDRCTRL_PSTAT` reader"]
pub type R = crate::R<DDRCTRL_PSTATrs>;
#[doc = "Field `RD_PORT_BUSY_0` reader - RD_PORT_BUSY_0"]
pub type RD_PORT_BUSY_0_R = crate::BitReader;
#[doc = "Field `RD_PORT_BUSY_1` reader - RD_PORT_BUSY_1"]
pub type RD_PORT_BUSY_1_R = crate::BitReader;
#[doc = "Field `WR_PORT_BUSY_0` reader - WR_PORT_BUSY_0"]
pub type WR_PORT_BUSY_0_R = crate::BitReader;
#[doc = "Field `WR_PORT_BUSY_1` reader - WR_PORT_BUSY_1"]
pub type WR_PORT_BUSY_1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RD_PORT_BUSY_0"]
    #[inline(always)]
    pub fn rd_port_busy_0(&self) -> RD_PORT_BUSY_0_R {
        RD_PORT_BUSY_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RD_PORT_BUSY_1"]
    #[inline(always)]
    pub fn rd_port_busy_1(&self) -> RD_PORT_BUSY_1_R {
        RD_PORT_BUSY_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - WR_PORT_BUSY_0"]
    #[inline(always)]
    pub fn wr_port_busy_0(&self) -> WR_PORT_BUSY_0_R {
        WR_PORT_BUSY_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WR_PORT_BUSY_1"]
    #[inline(always)]
    pub fn wr_port_busy_1(&self) -> WR_PORT_BUSY_1_R {
        WR_PORT_BUSY_1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "DDRCTRL port status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PSTATrs;
impl crate::RegisterSpec for DDRCTRL_PSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_PSTAT to value 0"]
impl crate::Resettable for DDRCTRL_PSTATrs {
    const RESET_VALUE: u32 = 0;
}
