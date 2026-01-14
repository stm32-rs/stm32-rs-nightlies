///Register `IASR` reader
pub type R = crate::R<IASRrs>;
///Field `CAEF` reader - configuration access error flag
pub type CAEF_R = crate::BitReader;
///Field `IAEF` reader - illegal access error flag
pub type IAEF_R = crate::BitReader;
impl R {
    ///Bit 0 - configuration access error flag
    #[inline(always)]
    pub fn caef(&self) -> CAEF_R {
        CAEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access error flag
    #[inline(always)]
    pub fn iaef(&self) -> IAEF_R {
        IAEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IASR")
            .field("caef", &self.caef())
            .field("iaef", &self.iaef())
            .finish()
    }
}
/**RISAF illegal access status register

You can [`read`](crate::Reg::read) this register and get [`iasr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:IASR)*/
pub struct IASRrs;
impl crate::RegisterSpec for IASRrs {
    type Ux = u32;
}
///`read()` method returns [`iasr::R`](R) reader structure
impl crate::Readable for IASRrs {}
///`reset()` method sets IASR to value 0
impl crate::Resettable for IASRrs {}
