///Register `LTDC_LCR` reader
pub type R = crate::R<LTDC_LCRrs>;
///Field `LNBR` reader - LNBR
pub type LNBR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - LNBR
    #[inline(always)]
    pub fn lnbr(&self) -> LNBR_R {
        LNBR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_LCR")
            .field("lnbr", &self.lnbr())
            .finish()
    }
}
/**LDTC layer count register

You can [`read`](crate::Reg::read) this register and get [`ltdc_lcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:LTDC_LCR)*/
pub struct LTDC_LCRrs;
impl crate::RegisterSpec for LTDC_LCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_lcr::R`](R) reader structure
impl crate::Readable for LTDC_LCRrs {}
///`reset()` method sets LTDC_LCR to value 0x02
impl crate::Resettable for LTDC_LCRrs {
    const RESET_VALUE: u32 = 0x02;
}
