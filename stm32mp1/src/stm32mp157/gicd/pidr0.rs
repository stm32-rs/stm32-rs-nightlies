///Register `PIDR0` reader
pub type R = crate::R<PIDR0rs>;
///Field `PIDR0` reader - PIDR0
pub type PIDR0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR0
    #[inline(always)]
    pub fn pidr0(&self) -> PIDR0_R {
        PIDR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR0")
            .field("pidr0", &self.pidr0())
            .finish()
    }
}
/**GICD peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR0)*/
pub struct PIDR0rs;
impl crate::RegisterSpec for PIDR0rs {
    type Ux = u32;
}
///`read()` method returns [`pidr0::R`](R) reader structure
impl crate::Readable for PIDR0rs {}
///`reset()` method sets PIDR0 to value 0x90
impl crate::Resettable for PIDR0rs {
    const RESET_VALUE: u32 = 0x90;
}
