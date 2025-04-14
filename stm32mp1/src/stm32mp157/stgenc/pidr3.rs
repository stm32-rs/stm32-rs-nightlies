///Register `PIDR3` reader
pub type R = crate::R<PIDR3rs>;
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
        f.debug_struct("PIDR3")
            .field("cmod", &self.cmod())
            .field("revand", &self.revand())
            .finish()
    }
}
/**STGENC peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR3)*/
pub struct PIDR3rs;
impl crate::RegisterSpec for PIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`pidr3::R`](R) reader structure
impl crate::Readable for PIDR3rs {}
///`reset()` method sets PIDR3 to value 0
impl crate::Resettable for PIDR3rs {}
