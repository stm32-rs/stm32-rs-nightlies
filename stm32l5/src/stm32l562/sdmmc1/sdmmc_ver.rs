///Register `SDMMC_VER` reader
pub type R = crate::R<SDMMC_VERrs>;
///Field `MINREV` reader - IP minor revision number.
pub type MINREV_R = crate::FieldReader;
///Field `MAJREV` reader - IP major revision number.
pub type MAJREV_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - IP minor revision number.
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - IP major revision number.
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_VER")
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
/**SDMMC IP version register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_VER)*/
pub struct SDMMC_VERrs;
impl crate::RegisterSpec for SDMMC_VERrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_ver::R`](R) reader structure
impl crate::Readable for SDMMC_VERrs {}
///`reset()` method sets SDMMC_VER to value 0x10
impl crate::Resettable for SDMMC_VERrs {
    const RESET_VALUE: u32 = 0x10;
}
