///Register `MANAESCIPHERTEXT1REG` reader
pub type R = crate::R<MANAESCIPHERTEXT1REGrs>;
///Field `AES` reader - Manual AES Cipher Text
pub type AES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Manual AES Cipher Text
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESCIPHERTEXT1REG")
            .field("aes", &self.aes())
            .finish()
    }
}
/**MANAESCIPHERTEXT1REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext1reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:MANAESCIPHERTEXT1REG)*/
pub struct MANAESCIPHERTEXT1REGrs;
impl crate::RegisterSpec for MANAESCIPHERTEXT1REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaesciphertext1reg::R`](R) reader structure
impl crate::Readable for MANAESCIPHERTEXT1REGrs {}
///`reset()` method sets MANAESCIPHERTEXT1REG to value 0
impl crate::Resettable for MANAESCIPHERTEXT1REGrs {}
