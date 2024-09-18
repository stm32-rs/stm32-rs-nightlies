///Register `GICD_TYPER` reader
pub type R = crate::R<GICD_TYPERrs>;
///Field `ITLINESNUMBER` reader - ITLINESNUMBER
pub type ITLINESNUMBER_R = crate::FieldReader;
///Field `CPUNUMBER` reader - CPUNUMBER
pub type CPUNUMBER_R = crate::FieldReader;
///Field `SECURITYEXTN` reader - SECURITYEXTN
pub type SECURITYEXTN_R = crate::BitReader;
///Field `LSPI` reader - LSPI
pub type LSPI_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - ITLINESNUMBER
    #[inline(always)]
    pub fn itlinesnumber(&self) -> ITLINESNUMBER_R {
        ITLINESNUMBER_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:7 - CPUNUMBER
    #[inline(always)]
    pub fn cpunumber(&self) -> CPUNUMBER_R {
        CPUNUMBER_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 10 - SECURITYEXTN
    #[inline(always)]
    pub fn securityextn(&self) -> SECURITYEXTN_R {
        SECURITYEXTN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:15 - LSPI
    #[inline(always)]
    pub fn lspi(&self) -> LSPI_R {
        LSPI_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_TYPER")
            .field("itlinesnumber", &self.itlinesnumber())
            .field("cpunumber", &self.cpunumber())
            .field("securityextn", &self.securityextn())
            .field("lspi", &self.lspi())
            .finish()
    }
}
/**GICD interrupt controller type register

You can [`read`](crate::Reg::read) this register and get [`gicd_typer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_TYPER)*/
pub struct GICD_TYPERrs;
impl crate::RegisterSpec for GICD_TYPERrs {
    type Ux = u32;
}
///`read()` method returns [`gicd_typer::R`](R) reader structure
impl crate::Readable for GICD_TYPERrs {}
///`reset()` method sets GICD_TYPER to value 0xfc28
impl crate::Resettable for GICD_TYPERrs {
    const RESET_VALUE: u32 = 0xfc28;
}
