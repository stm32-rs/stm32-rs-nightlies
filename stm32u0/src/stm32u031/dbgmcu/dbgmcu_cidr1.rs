///Register `DBGMCU_CIDR1` reader
pub type R = crate::R<DBGMCU_CIDR1rs>;
///Field `PREAMBLE` reader - component identification bits \[11:8\]
pub type PREAMBLE_R = crate::FieldReader;
/**Field `CLASS` reader - component identification bits \[15:12\]
- component class*/
pub type CLASS_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - component identification bits \[11:8\]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    /**Bits 4:7 - component identification bits \[15:12\]
    - component class*/
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_CIDR1")
            .field("preamble", &self.preamble())
            .field("class", &self.class())
            .finish()
    }
}
/**DBGMCU CoreSight component identity register 1

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_CIDR1)*/
pub struct DBGMCU_CIDR1rs;
impl crate::RegisterSpec for DBGMCU_CIDR1rs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_cidr1::R`](R) reader structure
impl crate::Readable for DBGMCU_CIDR1rs {}
///`reset()` method sets DBGMCU_CIDR1 to value 0xf0
impl crate::Resettable for DBGMCU_CIDR1rs {
    const RESET_VALUE: u32 = 0xf0;
}