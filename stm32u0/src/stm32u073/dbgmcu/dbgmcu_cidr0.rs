///Register `DBGMCU_CIDR0` reader
pub type R = crate::R<DBGMCU_CIDR0rs>;
///Field `PREAMBLE` reader - component identification bits \[7:0\]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - component identification bits \[7:0\]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_CIDR0")
            .field("preamble", &self.preamble())
            .finish()
    }
}
/**DBGMCU CoreSight component identity register 0

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:DBGMCU_CIDR0)*/
pub struct DBGMCU_CIDR0rs;
impl crate::RegisterSpec for DBGMCU_CIDR0rs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_cidr0::R`](R) reader structure
impl crate::Readable for DBGMCU_CIDR0rs {}
///`reset()` method sets DBGMCU_CIDR0 to value 0x0d
impl crate::Resettable for DBGMCU_CIDR0rs {
    const RESET_VALUE: u32 = 0x0d;
}
