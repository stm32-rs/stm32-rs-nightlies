///Register `ELSR0` reader
pub type R = crate::R<ELSR0rs>;
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
        f.debug_struct("ELSR0")
            .field("elsr0", &self.elsr0())
            .finish()
    }
}
/**GICH empty list status register

You can [`read`](crate::Reg::read) this register and get [`elsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICH:ELSR0)*/
pub struct ELSR0rs;
impl crate::RegisterSpec for ELSR0rs {
    type Ux = u32;
}
///`read()` method returns [`elsr0::R`](R) reader structure
impl crate::Readable for ELSR0rs {}
///`reset()` method sets ELSR0 to value 0x0f
impl crate::Resettable for ELSR0rs {
    const RESET_VALUE: u32 = 0x0f;
}
