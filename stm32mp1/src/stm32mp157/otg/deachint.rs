///Register `DEACHINT` reader
pub type R = crate::R<DEACHINTrs>;
///Field `IEP1INT` reader - IEP1INT
pub type IEP1INT_R = crate::BitReader;
///Field `OEP1INT` reader - OEP1INT
pub type OEP1INT_R = crate::BitReader;
impl R {
    ///Bit 1 - IEP1INT
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 17 - OEP1INT
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINT")
            .field("iep1int", &self.iep1int())
            .field("oep1int", &self.oep1int())
            .finish()
    }
}
/**OTG device each endpoint interrupt register

You can [`read`](crate::Reg::read) this register and get [`deachint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:DEACHINT)*/
pub struct DEACHINTrs;
impl crate::RegisterSpec for DEACHINTrs {
    type Ux = u32;
}
///`read()` method returns [`deachint::R`](R) reader structure
impl crate::Readable for DEACHINTrs {}
///`reset()` method sets DEACHINT to value 0
impl crate::Resettable for DEACHINTrs {}
