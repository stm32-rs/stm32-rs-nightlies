///Register `HSEM_VERR` reader
pub type R = crate::R<HSEM_VERRrs>;
///Field `MINREV` reader - MINREV
pub type MINREV_R = crate::FieldReader;
///Field `MAJREV` reader - MAJREV
pub type MAJREV_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - MINREV
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MAJREV
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_VERR")
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
/**HSEM IP version register

You can [`read`](crate::Reg::read) this register and get [`hsem_verr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:HSEM_VERR)*/
pub struct HSEM_VERRrs;
impl crate::RegisterSpec for HSEM_VERRrs {
    type Ux = u32;
}
///`read()` method returns [`hsem_verr::R`](R) reader structure
impl crate::Readable for HSEM_VERRrs {}
///`reset()` method sets HSEM_VERR to value 0x20
impl crate::Resettable for HSEM_VERRrs {
    const RESET_VALUE: u32 = 0x20;
}
