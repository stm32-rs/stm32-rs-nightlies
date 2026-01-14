///Register `ECCDR` reader
pub type R = crate::R<ECCDRrs>;
///Field `DATA_ECC` reader - ECC error data
pub type DATA_ECC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - ECC error data
    #[inline(always)]
    pub fn data_ecc(&self) -> DATA_ECC_R {
        DATA_ECC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCDR")
            .field("data_ecc", &self.data_ecc())
            .finish()
    }
}
/**FLASH ECC data

You can [`read`](crate::Reg::read) this register and get [`eccdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:ECCDR)*/
pub struct ECCDRrs;
impl crate::RegisterSpec for ECCDRrs {
    type Ux = u32;
}
///`read()` method returns [`eccdr::R`](R) reader structure
impl crate::Readable for ECCDRrs {}
///`reset()` method sets ECCDR to value 0
impl crate::Resettable for ECCDRrs {}
