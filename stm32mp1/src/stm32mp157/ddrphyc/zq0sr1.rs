///Register `ZQ0SR1` reader
pub type R = crate::R<ZQ0SR1rs>;
///Field `ZPD` reader - ZPD
pub type ZPD_R = crate::FieldReader;
///Field `ZPU` reader - ZPU
pub type ZPU_R = crate::FieldReader;
///Field `OPD` reader - OPD
pub type OPD_R = crate::FieldReader;
///Field `OPU` reader - OPU
pub type OPU_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - ZPD
    #[inline(always)]
    pub fn zpd(&self) -> ZPD_R {
        ZPD_R::new(self.bits & 3)
    }
    ///Bits 2:3 - ZPU
    #[inline(always)]
    pub fn zpu(&self) -> ZPU_R {
        ZPU_R::new((self.bits >> 2) & 3)
    }
    ///Bits 4:5 - OPD
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - OPU
    #[inline(always)]
    pub fn opu(&self) -> OPU_R {
        OPU_R::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQ0SR1")
            .field("zpd", &self.zpd())
            .field("zpu", &self.zpu())
            .field("opd", &self.opd())
            .field("opu", &self.opu())
            .finish()
    }
}
/**DDRPHYC ZQ0S register 1

You can [`read`](crate::Reg::read) this register and get [`zq0sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ZQ0SR1)*/
pub struct ZQ0SR1rs;
impl crate::RegisterSpec for ZQ0SR1rs {
    type Ux = u8;
}
///`read()` method returns [`zq0sr1::R`](R) reader structure
impl crate::Readable for ZQ0SR1rs {}
///`reset()` method sets ZQ0SR1 to value 0
impl crate::Resettable for ZQ0SR1rs {}
