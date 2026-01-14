///Register `DIR` reader
pub type R = crate::R<DIRrs>;
///Field `THI` reader - THI
pub type THI_R = crate::FieldReader<u16>;
///Field `TLO` reader - TLO
pub type TLO_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - THI
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - TLO
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR")
            .field("thi", &self.thi())
            .field("tlo", &self.tlo())
            .finish()
    }
}
/**Debug information register

You can [`read`](crate::Reg::read) this register and get [`dir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPDIFRX:DIR)*/
pub struct DIRrs;
impl crate::RegisterSpec for DIRrs {
    type Ux = u32;
}
///`read()` method returns [`dir::R`](R) reader structure
impl crate::Readable for DIRrs {}
///`reset()` method sets DIR to value 0
impl crate::Resettable for DIRrs {}
