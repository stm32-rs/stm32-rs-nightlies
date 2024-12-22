///Register `DBGMCU_PIDR4` reader
pub type R = crate::R<DBGMCU_PIDR4rs>;
///Field `JEP106CON` reader - JEP106 continuation code
pub type JEP106CON_R = crate::FieldReader;
///Field `SIZE` reader - register file size
pub type SIZE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - JEP106 continuation code
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - register file size
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_PIDR4")
            .field("jep106con", &self.jep106con())
            .field("size", &self.size())
            .finish()
    }
}
/**DBGMCU CoreSight peripheral identity register 4

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:DBGMCU_PIDR4)*/
pub struct DBGMCU_PIDR4rs;
impl crate::RegisterSpec for DBGMCU_PIDR4rs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_pidr4::R`](R) reader structure
impl crate::Readable for DBGMCU_PIDR4rs {}
///`reset()` method sets DBGMCU_PIDR4 to value 0
impl crate::Resettable for DBGMCU_PIDR4rs {
    const RESET_VALUE: u32 = 0;
}
