///Register `LPGPIO_IDR` reader
pub type R = crate::R<LPGPIO_IDRrs>;
///Field `IDy` reader - IDy
pub type IDY_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IDy
    #[inline(always)]
    pub fn idy(&self) -> IDY_R {
        IDY_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPGPIO_IDR")
            .field("idy", &self.idy())
            .finish()
    }
}
/**LPGPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`lpgpio_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPGPIO1:LPGPIO_IDR)*/
pub struct LPGPIO_IDRrs;
impl crate::RegisterSpec for LPGPIO_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`lpgpio_idr::R`](R) reader structure
impl crate::Readable for LPGPIO_IDRrs {}
///`reset()` method sets LPGPIO_IDR to value 0
impl crate::Resettable for LPGPIO_IDRrs {
    const RESET_VALUE: u32 = 0;
}
