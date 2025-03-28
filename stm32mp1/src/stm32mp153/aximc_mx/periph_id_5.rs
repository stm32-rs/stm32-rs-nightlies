///Register `PERIPH_ID_5` reader
pub type R = crate::R<PERIPH_ID_5rs>;
///Field `PERIPH_ID_5` reader - PERIPH_ID_5
pub type PERIPH_ID_5_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PERIPH_ID_5
    #[inline(always)]
    pub fn periph_id_5(&self) -> PERIPH_ID_5_R {
        PERIPH_ID_5_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPH_ID_5")
            .field("periph_id_5", &self.periph_id_5())
            .finish()
    }
}
/**AXIMC peripheral ID5 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_5)*/
pub struct PERIPH_ID_5rs;
impl crate::RegisterSpec for PERIPH_ID_5rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id_5::R`](R) reader structure
impl crate::Readable for PERIPH_ID_5rs {}
///`reset()` method sets PERIPH_ID_5 to value 0
impl crate::Resettable for PERIPH_ID_5rs {}
