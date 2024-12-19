///Register `RISR` reader
pub type R = crate::R<RISRrs>;
///Field `INRIS` reader - Input FIFO service raw interrupt status
pub type INRIS_R = crate::BitReader;
///Field `OUTRIS` reader - Output FIFO service raw interrupt status
pub type OUTRIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Input FIFO service raw interrupt status
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service raw interrupt status
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RISR")
            .field("outris", &self.outris())
            .field("inris", &self.inris())
            .finish()
    }
}
/**raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:RISR)*/
pub struct RISRrs;
impl crate::RegisterSpec for RISRrs {
    type Ux = u32;
}
///`read()` method returns [`risr::R`](R) reader structure
impl crate::Readable for RISRrs {}
///`reset()` method sets RISR to value 0x01
impl crate::Resettable for RISRrs {
    const RESET_VALUE: u32 = 0x01;
}
