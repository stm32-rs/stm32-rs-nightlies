///Register `CIDR2` reader
pub type R = crate::R<CIDR2rs>;
///Field `CIDR2` reader - CIDR2
pub type CIDR2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CIDR2
    #[inline(always)]
    pub fn cidr2(&self) -> CIDR2_R {
        CIDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIDR2")
            .field("cidr2", &self.cidr2())
            .finish()
    }
}
/**GICD component ID2 register

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:CIDR2)*/
pub struct CIDR2rs;
impl crate::RegisterSpec for CIDR2rs {
    type Ux = u32;
}
///`read()` method returns [`cidr2::R`](R) reader structure
impl crate::Readable for CIDR2rs {}
///`reset()` method sets CIDR2 to value 0x05
impl crate::Resettable for CIDR2rs {
    const RESET_VALUE: u32 = 0x05;
}
