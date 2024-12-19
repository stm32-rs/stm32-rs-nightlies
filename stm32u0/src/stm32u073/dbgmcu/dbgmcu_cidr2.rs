///Register `DBGMCU_CIDR2` reader
pub type R = crate::R<DBGMCU_CIDR2rs>;
///Field `PREAMBLE` reader - component identification bits \[23:16\]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - component identification bits \[23:16\]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_CIDR2")
            .field("preamble", &self.preamble())
            .finish()
    }
}
/**DBGMCU CoreSight component identity register 2

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:DBGMCU_CIDR2)*/
pub struct DBGMCU_CIDR2rs;
impl crate::RegisterSpec for DBGMCU_CIDR2rs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_cidr2::R`](R) reader structure
impl crate::Readable for DBGMCU_CIDR2rs {}
///`reset()` method sets DBGMCU_CIDR2 to value 0x05
impl crate::Resettable for DBGMCU_CIDR2rs {
    const RESET_VALUE: u32 = 0x05;
}
