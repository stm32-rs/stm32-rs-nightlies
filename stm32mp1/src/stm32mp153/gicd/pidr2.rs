///Register `PIDR2` reader
pub type R = crate::R<PIDR2rs>;
///Field `PIDR2` reader - PIDR2
pub type PIDR2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR2
    #[inline(always)]
    pub fn pidr2(&self) -> PIDR2_R {
        PIDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR2")
            .field("pidr2", &self.pidr2())
            .finish()
    }
}
/**GICD peripheral ID2 register

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:PIDR2)*/
pub struct PIDR2rs;
impl crate::RegisterSpec for PIDR2rs {
    type Ux = u32;
}
///`read()` method returns [`pidr2::R`](R) reader structure
impl crate::Readable for PIDR2rs {}
///`reset()` method sets PIDR2 to value 0x2b
impl crate::Resettable for PIDR2rs {
    const RESET_VALUE: u32 = 0x2b;
}
