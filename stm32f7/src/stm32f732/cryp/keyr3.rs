///Register `KEYR3` reader
pub type R = crate::R<KEYR3rs>;
///Field `KEYR3` reader - Data output register
pub type KEYR3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr3(&self) -> KEYR3_R {
        KEYR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR3")
            .field("keyr3", &self.keyr3())
            .finish()
    }
}
/**key register

You can [`read`](crate::Reg::read) this register and get [`keyr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#CRYP:KEYR3)*/
pub struct KEYR3rs;
impl crate::RegisterSpec for KEYR3rs {
    type Ux = u32;
}
///`read()` method returns [`keyr3::R`](R) reader structure
impl crate::Readable for KEYR3rs {}
///`reset()` method sets KEYR3 to value 0
impl crate::Resettable for KEYR3rs {}
