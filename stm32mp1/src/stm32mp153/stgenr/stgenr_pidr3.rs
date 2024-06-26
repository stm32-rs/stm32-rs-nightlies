///Register `STGENR_PIDR3` reader
pub type R = crate::R<STGENR_PIDR3rs>;
///Field `CMOD` reader - CMOD
pub type CMOD_R = crate::FieldReader;
///Field `REVAND` reader - REVAND
pub type REVAND_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CMOD
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - REVAND
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENR_PIDR3")
            .field("cmod", &self.cmod())
            .field("revand", &self.revand())
            .finish()
    }
}
/**STGENR peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`stgenr_pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENR:STGENR_PIDR3)*/
pub struct STGENR_PIDR3rs;
impl crate::RegisterSpec for STGENR_PIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`stgenr_pidr3::R`](R) reader structure
impl crate::Readable for STGENR_PIDR3rs {}
///`reset()` method sets STGENR_PIDR3 to value 0
impl crate::Resettable for STGENR_PIDR3rs {
    const RESET_VALUE: u32 = 0;
}
