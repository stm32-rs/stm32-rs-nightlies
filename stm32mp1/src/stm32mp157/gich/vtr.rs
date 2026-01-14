///Register `VTR` reader
pub type R = crate::R<VTRrs>;
///Field `LISTREGS` reader - LISTREGS
pub type LISTREGS_R = crate::FieldReader;
///Field `PREBITS` reader - PREBITS
pub type PREBITS_R = crate::FieldReader;
///Field `PRIBITS` reader - PRIBITS
pub type PRIBITS_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - LISTREGS
    #[inline(always)]
    pub fn listregs(&self) -> LISTREGS_R {
        LISTREGS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 26:28 - PREBITS
    #[inline(always)]
    pub fn prebits(&self) -> PREBITS_R {
        PREBITS_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - PRIBITS
    #[inline(always)]
    pub fn pribits(&self) -> PRIBITS_R {
        PRIBITS_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VTR")
            .field("listregs", &self.listregs())
            .field("prebits", &self.prebits())
            .field("pribits", &self.pribits())
            .finish()
    }
}
/**GICH VGIC type register

You can [`read`](crate::Reg::read) this register and get [`vtr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:VTR)*/
pub struct VTRrs;
impl crate::RegisterSpec for VTRrs {
    type Ux = u32;
}
///`read()` method returns [`vtr::R`](R) reader structure
impl crate::Readable for VTRrs {}
///`reset()` method sets VTR to value 0x9000_0003
impl crate::Resettable for VTRrs {
    const RESET_VALUE: u32 = 0x9000_0003;
}
