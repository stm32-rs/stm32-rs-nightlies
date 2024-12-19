///Register `DBGMCU_PIDR2` reader
pub type R = crate::R<DBGMCU_PIDR2rs>;
///Field `JEP106ID` reader - JEP106 identity code bits \[6:4\]
pub type JEP106ID_R = crate::FieldReader;
///Field `JEDEC` reader - JEDEC assigned value
pub type JEDEC_R = crate::BitReader;
///Field `REVISION` reader - component revision number
pub type REVISION_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - JEP106 identity code bits \[6:4\]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - JEDEC assigned value
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - component revision number
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_PIDR2")
            .field("jep106id", &self.jep106id())
            .field("jedec", &self.jedec())
            .field("revision", &self.revision())
            .finish()
    }
}
/**DBGMCU CoreSight peripheral identity register 2

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DBGMCU:DBGMCU_PIDR2)*/
pub struct DBGMCU_PIDR2rs;
impl crate::RegisterSpec for DBGMCU_PIDR2rs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_pidr2::R`](R) reader structure
impl crate::Readable for DBGMCU_PIDR2rs {}
///`reset()` method sets DBGMCU_PIDR2 to value 0x0a
impl crate::Resettable for DBGMCU_PIDR2rs {
    const RESET_VALUE: u32 = 0x0a;
}
