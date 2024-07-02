///Register `GICD_CIDR3` reader
pub type R = crate::R<GICD_CIDR3rs>;
///Field `CIDR3` reader - CIDR3
pub type CIDR3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CIDR3
    #[inline(always)]
    pub fn cidr3(&self) -> CIDR3_R {
        CIDR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR3")
            .field("cidr3", &self.cidr3())
            .finish()
    }
}
/**GICD component ID3 register

You can [`read`](crate::Reg::read) this register and get [`gicd_cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_CIDR3)*/
pub struct GICD_CIDR3rs;
impl crate::RegisterSpec for GICD_CIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_cidr3::R`](R) reader structure
impl crate::Readable for GICD_CIDR3rs {}
///`reset()` method sets GICD_CIDR3 to value 0xb1
impl crate::Resettable for GICD_CIDR3rs {
    const RESET_VALUE: u32 = 0xb1;
}
