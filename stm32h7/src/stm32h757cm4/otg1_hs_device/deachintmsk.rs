///Register `DEACHINTMSK` reader
pub type R = crate::R<DEACHINTMSKrs>;
///Register `DEACHINTMSK` writer
pub type W = crate::W<DEACHINTMSKrs>;
///Field `IEP1INTM` reader - IN Endpoint 1 interrupt mask bit
pub type IEP1INTM_R = crate::BitReader;
///Field `IEP1INTM` writer - IN Endpoint 1 interrupt mask bit
pub type IEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OEP1INTM` reader - OUT Endpoint 1 interrupt mask bit
pub type OEP1INTM_R = crate::BitReader;
///Field `OEP1INTM` writer - OUT Endpoint 1 interrupt mask bit
pub type OEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - IN Endpoint 1 interrupt mask bit
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 17 - OUT Endpoint 1 interrupt mask bit
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINTMSK")
            .field("iep1intm", &self.iep1intm())
            .field("oep1intm", &self.oep1intm())
            .finish()
    }
}
impl W {
    ///Bit 1 - IN Endpoint 1 interrupt mask bit
    #[inline(always)]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<'_, DEACHINTMSKrs> {
        IEP1INTM_W::new(self, 1)
    }
    ///Bit 17 - OUT Endpoint 1 interrupt mask bit
    #[inline(always)]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<'_, DEACHINTMSKrs> {
        OEP1INTM_W::new(self, 17)
    }
}
/**OTG_HS device each endpoint interrupt register mask

You can [`read`](crate::Reg::read) this register and get [`deachintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#OTG1_HS_DEVICE:DEACHINTMSK)*/
pub struct DEACHINTMSKrs;
impl crate::RegisterSpec for DEACHINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`deachintmsk::R`](R) reader structure
impl crate::Readable for DEACHINTMSKrs {}
///`write(|w| ..)` method takes [`deachintmsk::W`](W) writer structure
impl crate::Writable for DEACHINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEACHINTMSK to value 0
impl crate::Resettable for DEACHINTMSKrs {}
