///Register `JTAGINR` reader
pub type R = crate::R<JTAGINRrs>;
///Field `JDATAIN` reader - JTAG input data
pub type JDATAIN_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - JTAG input data
    #[inline(always)]
    pub fn jdatain(&self) -> JDATAIN_R {
        JDATAIN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAGINR")
            .field("jdatain", &self.jdatain())
            .finish()
    }
}
/**BSEC JTAG input register

You can [`read`](crate::Reg::read) this register and get [`jtaginr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:JTAGINR)*/
pub struct JTAGINRrs;
impl crate::RegisterSpec for JTAGINRrs {
    type Ux = u32;
}
///`read()` method returns [`jtaginr::R`](R) reader structure
impl crate::Readable for JTAGINRrs {}
///`reset()` method sets JTAGINR to value 0
impl crate::Resettable for JTAGINRrs {}
