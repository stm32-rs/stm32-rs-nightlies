///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RTT4B` reader - FIFO is ready to transfer four bytes
pub type RTT4B_R = crate::BitReader;
///Field `RTT1B` reader - FIFO is ready to transfer one byte
pub type RTT1B_R = crate::BitReader;
impl R {
    ///Bit 2 - FIFO is ready to transfer four bytes
    #[inline(always)]
    pub fn rtt4b(&self) -> RTT4B_R {
        RTT4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO is ready to transfer one byte
    #[inline(always)]
    pub fn rtt1b(&self) -> RTT1B_R {
        RTT1B_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rtt4b", &self.rtt4b())
            .field("rtt1b", &self.rtt1b())
            .finish()
    }
}
/**PSSI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#PSSI:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
