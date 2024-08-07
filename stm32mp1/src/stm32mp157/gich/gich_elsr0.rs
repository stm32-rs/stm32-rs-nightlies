///Register `GICH_ELSR0` reader
pub type R = crate::R<GICH_ELSR0rs>;
///Field `ELSR0` reader - ELSR0
pub type ELSR0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ELSR0
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICH_ELSR0")
            .field("elsr0", &self.elsr0())
            .finish()
    }
}
/**GICH empty list status register

You can [`read`](crate::Reg::read) this register and get [`gich_elsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:GICH_ELSR0)*/
pub struct GICH_ELSR0rs;
impl crate::RegisterSpec for GICH_ELSR0rs {
    type Ux = u32;
}
///`read()` method returns [`gich_elsr0::R`](R) reader structure
impl crate::Readable for GICH_ELSR0rs {}
///`reset()` method sets GICH_ELSR0 to value 0x0f
impl crate::Resettable for GICH_ELSR0rs {
    const RESET_VALUE: u32 = 0x0f;
}
