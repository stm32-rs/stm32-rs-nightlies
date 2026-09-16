///Register `MRSTAT` reader
pub type R = crate::R<MRSTATrs>;
///Field `MR_WR_BUSY` reader - MR_WR_BUSY
pub type MR_WR_BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - MR_WR_BUSY
    #[inline(always)]
    pub fn mr_wr_busy(&self) -> MR_WR_BUSY_R {
        MR_WR_BUSY_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRSTAT")
            .field("mr_wr_busy", &self.mr_wr_busy())
            .finish()
    }
}
/**DDRCTRL mode register read/write status register

You can [`read`](crate::Reg::read) this register and get [`mrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:MRSTAT)*/
pub struct MRSTATrs;
impl crate::RegisterSpec for MRSTATrs {
    type Ux = u32;
}
///`read()` method returns [`mrstat::R`](R) reader structure
impl crate::Readable for MRSTATrs {}
///`reset()` method sets MRSTAT to value 0
impl crate::Resettable for MRSTATrs {}
