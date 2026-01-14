///Register `MANAESCIPHERTEXT3REG` reader
pub type R = crate::R<MANAESCIPHERTEXT3REGrs>;
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
        f.debug_struct("MANAESCIPHERTEXT3REG")
            .field("aes", &self.aes())
            .finish()
    }
}
/**MANAESCIPHERTEXT3REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext3reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCIPHERTEXT3REG)*/
pub struct MANAESCIPHERTEXT3REGrs;
impl crate::RegisterSpec for MANAESCIPHERTEXT3REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaesciphertext3reg::R`](R) reader structure
impl crate::Readable for MANAESCIPHERTEXT3REGrs {}
///`reset()` method sets MANAESCIPHERTEXT3REG to value 0
impl crate::Resettable for MANAESCIPHERTEXT3REGrs {}
