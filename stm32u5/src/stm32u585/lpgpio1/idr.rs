///Register `IDR` reader
pub type R = crate::R<IDRrs>;
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
        f.debug_struct("IDR").field("idy", &self.idy()).finish()
    }
}
/**LPGPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
