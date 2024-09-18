///Register `IPCC_VER` reader
pub type R = crate::R<IPCC_VERrs>;
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
        f.debug_struct("IPCC_VER")
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
/**IPCC IP Version register

You can [`read`](crate::Reg::read) this register and get [`ipcc_ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_VER)*/
pub struct IPCC_VERrs;
impl crate::RegisterSpec for IPCC_VERrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_ver::R`](R) reader structure
impl crate::Readable for IPCC_VERrs {}
///`reset()` method sets IPCC_VER to value 0x10
impl crate::Resettable for IPCC_VERrs {
    const RESET_VALUE: u32 = 0x10;
}
