///Register `ECCR3` reader
pub type R = crate::R<ECCR3rs>;
///Field `ECCx` reader - ECCx
pub type ECCX_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECCx
    #[inline(always)]
    pub fn eccx(&self) -> ECCX_R {
        ECCX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR3").field("eccx", &self.eccx()).finish()
    }
}
/**ECC result register 3

You can [`read`](crate::Reg::read) this register and get [`eccr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#FSMC:ECCR3)*/
pub struct ECCR3rs;
impl crate::RegisterSpec for ECCR3rs {
    type Ux = u32;
}
///`read()` method returns [`eccr3::R`](R) reader structure
impl crate::Readable for ECCR3rs {}
///`reset()` method sets ECCR3 to value 0
impl crate::Resettable for ECCR3rs {}
