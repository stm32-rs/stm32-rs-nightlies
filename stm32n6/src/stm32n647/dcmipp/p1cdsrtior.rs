///Register `P1CDSRTIOR` reader
pub type R = crate::R<P1CDSRTIORrs>;
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
        f.debug_struct("P1CDSRTIOR")
            .field("hratio", &self.hratio())
            .field("vratio", &self.vratio())
            .finish()
    }
}
/**DCMIPP Pipex current downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p1cdsrtior::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CDSRTIOR)*/
pub struct P1CDSRTIORrs;
impl crate::RegisterSpec for P1CDSRTIORrs {
    type Ux = u32;
}
///`read()` method returns [`p1cdsrtior::R`](R) reader structure
impl crate::Readable for P1CDSRTIORrs {}
///`reset()` method sets P1CDSRTIOR to value 0
impl crate::Resettable for P1CDSRTIORrs {}
