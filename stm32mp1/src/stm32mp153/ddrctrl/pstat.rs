///Register `PSTAT` reader
pub type R = crate::R<PSTATrs>;
///Field `RD_PORT_BUSY_0` reader - RD_PORT_BUSY_0
pub type RD_PORT_BUSY_0_R = crate::BitReader;
///Field `RD_PORT_BUSY_1` reader - RD_PORT_BUSY_1
pub type RD_PORT_BUSY_1_R = crate::BitReader;
///Field `WR_PORT_BUSY_0` reader - WR_PORT_BUSY_0
pub type WR_PORT_BUSY_0_R = crate::BitReader;
///Field `WR_PORT_BUSY_1` reader - WR_PORT_BUSY_1
pub type WR_PORT_BUSY_1_R = crate::BitReader;
impl R {
    ///Bit 0 - RD_PORT_BUSY_0
    #[inline(always)]
    pub fn rd_port_busy_0(&self) -> RD_PORT_BUSY_0_R {
        RD_PORT_BUSY_0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RD_PORT_BUSY_1
    #[inline(always)]
    pub fn rd_port_busy_1(&self) -> RD_PORT_BUSY_1_R {
        RD_PORT_BUSY_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - WR_PORT_BUSY_0
    #[inline(always)]
    pub fn wr_port_busy_0(&self) -> WR_PORT_BUSY_0_R {
        WR_PORT_BUSY_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - WR_PORT_BUSY_1
    #[inline(always)]
    pub fn wr_port_busy_1(&self) -> WR_PORT_BUSY_1_R {
        WR_PORT_BUSY_1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSTAT")
            .field("rd_port_busy_0", &self.rd_port_busy_0())
            .field("rd_port_busy_1", &self.rd_port_busy_1())
            .field("wr_port_busy_0", &self.wr_port_busy_0())
            .field("wr_port_busy_1", &self.wr_port_busy_1())
            .finish()
    }
}
/**DDRCTRL port status register

You can [`read`](crate::Reg::read) this register and get [`pstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PSTAT)*/
pub struct PSTATrs;
impl crate::RegisterSpec for PSTATrs {
    type Ux = u32;
}
///`read()` method returns [`pstat::R`](R) reader structure
impl crate::Readable for PSTATrs {}
///`reset()` method sets PSTAT to value 0
impl crate::Resettable for PSTATrs {}
