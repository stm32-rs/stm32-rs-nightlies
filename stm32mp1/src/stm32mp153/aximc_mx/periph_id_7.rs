///Register `PERIPH_ID_7` reader
pub type R = crate::R<PERIPH_ID_7rs>;
///Field `PERIPH_ID_7` reader - PERIPH_ID_7
pub type PERIPH_ID_7_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PERIPH_ID_7
    #[inline(always)]
    pub fn periph_id_7(&self) -> PERIPH_ID_7_R {
        PERIPH_ID_7_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPH_ID_7")
            .field("periph_id_7", &self.periph_id_7())
            .finish()
    }
}
/**AXIMC peripheral ID7 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_7)*/
pub struct PERIPH_ID_7rs;
impl crate::RegisterSpec for PERIPH_ID_7rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id_7::R`](R) reader structure
impl crate::Readable for PERIPH_ID_7rs {}
///`reset()` method sets PERIPH_ID_7 to value 0
impl crate::Resettable for PERIPH_ID_7rs {}
