///Register `CIDR3` reader
pub type R = crate::R<CIDR3rs>;
///Field `PRMBL_3` reader - PRMBL_3
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PRMBL_3
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIDR3")
            .field("prmbl_3", &self.prmbl_3())
            .finish()
    }
}
/**STGENR component ID3 register

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENR:CIDR3)*/
pub struct CIDR3rs;
impl crate::RegisterSpec for CIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`cidr3::R`](R) reader structure
impl crate::Readable for CIDR3rs {}
///`reset()` method sets CIDR3 to value 0xb1
impl crate::Resettable for CIDR3rs {
    const RESET_VALUE: u32 = 0xb1;
}
