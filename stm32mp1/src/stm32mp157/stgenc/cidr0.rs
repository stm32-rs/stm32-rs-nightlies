///Register `CIDR0` reader
pub type R = crate::R<CIDR0rs>;
///Field `PRMBL_0` reader - PRMBL_0
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PRMBL_0
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIDR0")
            .field("prmbl_0", &self.prmbl_0())
            .finish()
    }
}
/**STGENC component ID0 register

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CIDR0)*/
pub struct CIDR0rs;
impl crate::RegisterSpec for CIDR0rs {
    type Ux = u32;
}
///`read()` method returns [`cidr0::R`](R) reader structure
impl crate::Readable for CIDR0rs {}
///`reset()` method sets CIDR0 to value 0x0d
impl crate::Resettable for CIDR0rs {
    const RESET_VALUE: u32 = 0x0d;
}
