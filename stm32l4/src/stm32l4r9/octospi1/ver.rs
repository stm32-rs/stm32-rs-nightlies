///Register `VER` reader
pub type R = crate::R<VERrs>;
///Field `VER` reader - Version
pub type VER_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Version
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER").field("ver", &self.ver()).finish()
    }
}
/**version register

You can [`read`](crate::Reg::read) this register and get [`ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#OCTOSPI1:VER)*/
pub struct VERrs;
impl crate::RegisterSpec for VERrs {
    type Ux = u32;
}
///`read()` method returns [`ver::R`](R) reader structure
impl crate::Readable for VERrs {}
///`reset()` method sets VER to value 0x10
impl crate::Resettable for VERrs {
    const RESET_VALUE: u32 = 0x10;
}
