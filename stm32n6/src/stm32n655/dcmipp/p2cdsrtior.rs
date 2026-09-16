///Register `P2CDSRTIOR` reader
pub type R = crate::R<P2CDSRTIORrs>;
///Field `HRATIO` reader - Current horizontal ratio, from 8192 (1x) to 65535 (8x)
pub type HRATIO_R = crate::FieldReader<u16>;
///Field `VRATIO` reader - Current vertical ratio, from 8192 (1x) to 65535 (8x)
pub type VRATIO_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Current horizontal ratio, from 8192 (1x) to 65535 (8x)
    #[inline(always)]
    pub fn hratio(&self) -> HRATIO_R {
        HRATIO_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Current vertical ratio, from 8192 (1x) to 65535 (8x)
    #[inline(always)]
    pub fn vratio(&self) -> VRATIO_R {
        VRATIO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2CDSRTIOR")
            .field("hratio", &self.hratio())
            .field("vratio", &self.vratio())
            .finish()
    }
}
/**DCMIPP Pipex current downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p2cdsrtior::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P2CDSRTIOR)*/
pub struct P2CDSRTIORrs;
impl crate::RegisterSpec for P2CDSRTIORrs {
    type Ux = u32;
}
///`read()` method returns [`p2cdsrtior::R`](R) reader structure
impl crate::Readable for P2CDSRTIORrs {}
///`reset()` method sets P2CDSRTIOR to value 0
impl crate::Resettable for P2CDSRTIORrs {}
