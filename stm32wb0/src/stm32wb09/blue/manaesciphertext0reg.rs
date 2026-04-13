///Register `MANAESCIPHERTEXT0REG` reader
pub type R = crate::R<MANAESCIPHERTEXT0REGrs>;
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
        f.debug_struct("MANAESCIPHERTEXT0REG")
            .field("aes", &self.aes())
            .finish()
    }
}
/**MANAESCIPHERTEXT0REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext0reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#BLUE:MANAESCIPHERTEXT0REG)*/
pub struct MANAESCIPHERTEXT0REGrs;
impl crate::RegisterSpec for MANAESCIPHERTEXT0REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaesciphertext0reg::R`](R) reader structure
impl crate::Readable for MANAESCIPHERTEXT0REGrs {}
///`reset()` method sets MANAESCIPHERTEXT0REG to value 0
impl crate::Resettable for MANAESCIPHERTEXT0REGrs {}
