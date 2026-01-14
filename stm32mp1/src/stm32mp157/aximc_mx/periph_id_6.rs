///Register `PERIPH_ID_6` reader
pub type R = crate::R<PERIPH_ID_6rs>;
///Field `PERIPH_ID_6` reader - PERIPH_ID_6
pub type PERIPH_ID_6_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PERIPH_ID_6
    #[inline(always)]
    pub fn periph_id_6(&self) -> PERIPH_ID_6_R {
        PERIPH_ID_6_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPH_ID_6")
            .field("periph_id_6", &self.periph_id_6())
            .finish()
    }
}
/**AXIMC peripheral ID6 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#AXIMC_Mx:PERIPH_ID_6)*/
pub struct PERIPH_ID_6rs;
impl crate::RegisterSpec for PERIPH_ID_6rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id_6::R`](R) reader structure
impl crate::Readable for PERIPH_ID_6rs {}
///`reset()` method sets PERIPH_ID_6 to value 0
impl crate::Resettable for PERIPH_ID_6rs {}
