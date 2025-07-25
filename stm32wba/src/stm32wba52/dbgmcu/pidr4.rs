///Register `PIDR4` reader
pub type R = crate::R<PIDR4rs>;
///Field `JEP106CON` reader - JEP106 continuation code
pub type JEP106CON_R = crate::FieldReader;
///Field `F4KCOUNT` reader - Register file size
pub type F4KCOUNT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - JEP106 continuation code
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Register file size
    #[inline(always)]
    pub fn f4kcount(&self) -> F4KCOUNT_R {
        F4KCOUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR4")
            .field("jep106con", &self.jep106con())
            .field("f4kcount", &self.f4kcount())
            .finish()
    }
}
/**DBGMCU CoreSight peripheral identity register 4

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#DBGMCU:PIDR4)*/
pub struct PIDR4rs;
impl crate::RegisterSpec for PIDR4rs {
    type Ux = u32;
}
///`read()` method returns [`pidr4::R`](R) reader structure
impl crate::Readable for PIDR4rs {}
///`reset()` method sets PIDR4 to value 0
impl crate::Resettable for PIDR4rs {}
