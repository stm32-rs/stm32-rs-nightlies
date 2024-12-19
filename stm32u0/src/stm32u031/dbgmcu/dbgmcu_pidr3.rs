///Register `DBGMCU_PIDR3` reader
pub type R = crate::R<DBGMCU_PIDR3rs>;
///Field `CMOD` reader - customer modified
pub type CMOD_R = crate::FieldReader;
///Field `REVAND` reader - metal fix version
pub type REVAND_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - customer modified
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - metal fix version
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_PIDR3")
            .field("cmod", &self.cmod())
            .field("revand", &self.revand())
            .finish()
    }
}
/**DBGMCU CoreSight peripheral identity register 3

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_PIDR3)*/
pub struct DBGMCU_PIDR3rs;
impl crate::RegisterSpec for DBGMCU_PIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_pidr3::R`](R) reader structure
impl crate::Readable for DBGMCU_PIDR3rs {}
///`reset()` method sets DBGMCU_PIDR3 to value 0
impl crate::Resettable for DBGMCU_PIDR3rs {
    const RESET_VALUE: u32 = 0;
}
