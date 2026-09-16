///Register `EPOCHSR` reader
pub type R = crate::R<EPOCHSRrs>;
///Field `EPOCH` reader - Epoch This value is distributed by hardware to the SAES peripheral.
pub type EPOCH_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Epoch This value is distributed by hardware to the SAES peripheral.
    #[inline(always)]
    pub fn epoch(&self) -> EPOCH_R {
        EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPOCHSR")
            .field("epoch", &self.epoch())
            .finish()
    }
}
/**FLASH epoch status register

You can [`read`](crate::Reg::read) this register and get [`epochsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:EPOCHSR)*/
pub struct EPOCHSRrs;
impl crate::RegisterSpec for EPOCHSRrs {
    type Ux = u32;
}
///`read()` method returns [`epochsr::R`](R) reader structure
impl crate::Readable for EPOCHSRrs {}
///`reset()` method sets EPOCHSR to value 0
impl crate::Resettable for EPOCHSRrs {}
