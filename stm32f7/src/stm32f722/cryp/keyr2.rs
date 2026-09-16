///Register `KEYR2` reader
pub type R = crate::R<KEYR2rs>;
///Field `KEYR2` reader - Data output register
pub type KEYR2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:30 - Data output register
    #[inline(always)]
    pub fn keyr2(&self) -> KEYR2_R {
        KEYR2_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR2")
            .field("keyr2", &self.keyr2())
            .finish()
    }
}
/**key register

You can [`read`](crate::Reg::read) this register and get [`keyr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#CRYP:KEYR2)*/
pub struct KEYR2rs;
impl crate::RegisterSpec for KEYR2rs {
    type Ux = u32;
}
///`read()` method returns [`keyr2::R`](R) reader structure
impl crate::Readable for KEYR2rs {}
///`reset()` method sets KEYR2 to value 0
impl crate::Resettable for KEYR2rs {}
