///Register `CIDR1` reader
pub type R = crate::R<CIDR1rs>;
///Field `CIDR1` reader - CIDR1
pub type CIDR1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CIDR1
    #[inline(always)]
    pub fn cidr1(&self) -> CIDR1_R {
        CIDR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIDR1")
            .field("cidr1", &self.cidr1())
            .finish()
    }
}
/**GICD component ID1 register

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CIDR1)*/
pub struct CIDR1rs;
impl crate::RegisterSpec for CIDR1rs {
    type Ux = u32;
}
///`read()` method returns [`cidr1::R`](R) reader structure
impl crate::Readable for CIDR1rs {}
///`reset()` method sets CIDR1 to value 0xf0
impl crate::Resettable for CIDR1rs {
    const RESET_VALUE: u32 = 0xf0;
}
