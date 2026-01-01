///Register `POISONSTAT` reader
pub type R = crate::R<POISONSTATrs>;
///Field `WR_POISON_INTR_0` reader - WR_POISON_INTR_0
pub type WR_POISON_INTR_0_R = crate::BitReader;
///Field `WR_POISON_INTR_1` reader - WR_POISON_INTR_1
pub type WR_POISON_INTR_1_R = crate::BitReader;
///Field `RD_POISON_INTR_0` reader - RD_POISON_INTR_0
pub type RD_POISON_INTR_0_R = crate::BitReader;
///Field `RD_POISON_INTR_1` reader - RD_POISON_INTR_1
pub type RD_POISON_INTR_1_R = crate::BitReader;
impl R {
    ///Bit 0 - WR_POISON_INTR_0
    #[inline(always)]
    pub fn wr_poison_intr_0(&self) -> WR_POISON_INTR_0_R {
        WR_POISON_INTR_0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WR_POISON_INTR_1
    #[inline(always)]
    pub fn wr_poison_intr_1(&self) -> WR_POISON_INTR_1_R {
        WR_POISON_INTR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - RD_POISON_INTR_0
    #[inline(always)]
    pub fn rd_poison_intr_0(&self) -> RD_POISON_INTR_0_R {
        RD_POISON_INTR_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RD_POISON_INTR_1
    #[inline(always)]
    pub fn rd_poison_intr_1(&self) -> RD_POISON_INTR_1_R {
        RD_POISON_INTR_1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POISONSTAT")
            .field("wr_poison_intr_0", &self.wr_poison_intr_0())
            .field("wr_poison_intr_1", &self.wr_poison_intr_1())
            .field("rd_poison_intr_0", &self.rd_poison_intr_0())
            .field("rd_poison_intr_1", &self.rd_poison_intr_1())
            .finish()
    }
}
/**DDRCTRL AXI Poison status register

You can [`read`](crate::Reg::read) this register and get [`poisonstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:POISONSTAT)*/
pub struct POISONSTATrs;
impl crate::RegisterSpec for POISONSTATrs {
    type Ux = u32;
}
///`read()` method returns [`poisonstat::R`](R) reader structure
impl crate::Readable for POISONSTATrs {}
///`reset()` method sets POISONSTAT to value 0
impl crate::Resettable for POISONSTATrs {}
