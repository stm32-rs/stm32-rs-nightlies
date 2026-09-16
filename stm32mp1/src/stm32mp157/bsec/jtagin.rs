///Register `JTAGIN` reader
pub type R = crate::R<JTAGINrs>;
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAGIN")
            .field("data", &self.data())
            .finish()
    }
}
/**BSEC JTAG input register

You can [`read`](crate::Reg::read) this register and get [`jtagin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:JTAGIN)*/
pub struct JTAGINrs;
impl crate::RegisterSpec for JTAGINrs {
    type Ux = u32;
}
///`read()` method returns [`jtagin::R`](R) reader structure
impl crate::Readable for JTAGINrs {}
///`reset()` method sets JTAGIN to value 0
impl crate::Resettable for JTAGINrs {}
