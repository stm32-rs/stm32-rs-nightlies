///Register `DEACHINT` reader
pub type R = crate::R<DEACHINTrs>;
///Register `DEACHINT` writer
pub type W = crate::W<DEACHINTrs>;
///Field `IEP1INT` reader - IN endpoint 1interrupt bit
pub type IEP1INT_R = crate::BitReader;
///Field `IEP1INT` writer - IN endpoint 1interrupt bit
pub type IEP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OEP1INT` reader - OUT endpoint 1 interrupt bit
pub type OEP1INT_R = crate::BitReader;
///Field `OEP1INT` writer - OUT endpoint 1 interrupt bit
pub type OEP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - IN endpoint 1interrupt bit
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 17 - OUT endpoint 1 interrupt bit
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
impl W {
    ///Bit 1 - IN endpoint 1interrupt bit
    #[inline(always)]
    pub fn iep1int(&mut self) -> IEP1INT_W<DEACHINTrs> {
        IEP1INT_W::new(self, 1)
    }
    ///Bit 17 - OUT endpoint 1 interrupt bit
    #[inline(always)]
    pub fn oep1int(&mut self) -> OEP1INT_W<DEACHINTrs> {
        OEP1INT_W::new(self, 17)
    }
}
/**OTG_HS device each endpoint interrupt register

You can [`read`](crate::Reg::read) this register and get [`deachint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#OTG_HS_DEVICE:DEACHINT)*/
pub struct DEACHINTrs;
impl crate::RegisterSpec for DEACHINTrs {
    type Ux = u32;
}
///`read()` method returns [`deachint::R`](R) reader structure
impl crate::Readable for DEACHINTrs {}
///`write(|w| ..)` method takes [`deachint::W`](W) writer structure
impl crate::Writable for DEACHINTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEACHINT to value 0
impl crate::Resettable for DEACHINTrs {}
