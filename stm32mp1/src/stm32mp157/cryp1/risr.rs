///Register `RISR` reader
pub type R = crate::R<RISRrs>;
///Field `INRIS` reader - INRIS
pub type INRIS_R = crate::BitReader;
///Field `OUTRIS` reader - OUTRIS
pub type OUTRIS_R = crate::BitReader;
impl R {
    ///Bit 0 - INRIS
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OUTRIS
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RISR")
            .field("inris", &self.inris())
            .field("outris", &self.outris())
            .finish()
    }
}
/**The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.

You can [`read`](crate::Reg::read) this register and get [`risr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:RISR)*/
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
