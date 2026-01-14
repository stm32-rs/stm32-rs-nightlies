///Register `VERR` reader
pub type R = crate::R<VERRrs>;
///Field `MINREV` reader - GPIO minor revision
pub type MINREV_R = crate::FieldReader;
///Field `MAJREV` reader - GPIO major revision
pub type MAJREV_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - GPIO minor revision
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - GPIO major revision
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERR")
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
/**GPIO port Q version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPIOQ:VERR)*/
pub struct VERRrs;
impl crate::RegisterSpec for VERRrs {
    type Ux = u32;
}
///`read()` method returns [`verr::R`](R) reader structure
impl crate::Readable for VERRrs {}
///`reset()` method sets VERR to value 0x10
impl crate::Resettable for VERRrs {
    const RESET_VALUE: u32 = 0x10;
}
