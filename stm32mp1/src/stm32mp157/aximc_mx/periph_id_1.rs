///Register `PERIPH_ID_1` reader
pub type R = crate::R<PERIPH_ID_1rs>;
///Field `PERIPH_ID_1` reader - PERIPH_ID_1
pub type PERIPH_ID_1_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PERIPH_ID_1
    #[inline(always)]
    pub fn periph_id_1(&self) -> PERIPH_ID_1_R {
        PERIPH_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPH_ID_1")
            .field("periph_id_1", &self.periph_id_1())
            .finish()
    }
}
/**AXIMC peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#AXIMC_Mx:PERIPH_ID_1)*/
pub struct PERIPH_ID_1rs;
impl crate::RegisterSpec for PERIPH_ID_1rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id_1::R`](R) reader structure
impl crate::Readable for PERIPH_ID_1rs {}
///`reset()` method sets PERIPH_ID_1 to value 0xb4
impl crate::Resettable for PERIPH_ID_1rs {
    const RESET_VALUE: u32 = 0xb4;
}
