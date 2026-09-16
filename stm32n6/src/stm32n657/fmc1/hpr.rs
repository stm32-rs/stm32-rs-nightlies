///Register `HPR` reader
pub type R = crate::R<HPRrs>;
///Field `HPR` reader - Hamming parity result
pub type HPR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hamming parity result
    #[inline(always)]
    pub fn hpr(&self) -> HPR_R {
        HPR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPR").field("hpr", &self.hpr()).finish()
    }
}
/**FMC Hamming parity result registers

You can [`read`](crate::Reg::read) this register and get [`hpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:HPR)*/
pub struct HPRrs;
impl crate::RegisterSpec for HPRrs {
    type Ux = u32;
}
///`read()` method returns [`hpr::R`](R) reader structure
impl crate::Readable for HPRrs {}
///`reset()` method sets HPR to value 0
impl crate::Resettable for HPRrs {}
