///Register `PIDR3` reader
pub type R = crate::R<PIDR3rs>;
///Field `PIDR3` reader - PIDR3
pub type PIDR3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR3
    #[inline(always)]
    pub fn pidr3(&self) -> PIDR3_R {
        PIDR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR3")
            .field("pidr3", &self.pidr3())
            .finish()
    }
}
/**GICD peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:PIDR3)*/
pub struct PIDR3rs;
impl crate::RegisterSpec for PIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`pidr3::R`](R) reader structure
impl crate::Readable for PIDR3rs {}
///`reset()` method sets PIDR3 to value 0
impl crate::Resettable for PIDR3rs {}
