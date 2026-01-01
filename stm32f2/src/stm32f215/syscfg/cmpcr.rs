///Register `CMPCR` reader
pub type R = crate::R<CMPCRrs>;
///Register `CMPCR` writer
pub type W = crate::W<CMPCRrs>;
///Field `CMP_PD` reader - Compensation cell power-down
pub type CMP_PD_R = crate::BitReader;
///Field `CMP_PD` writer - Compensation cell power-down
pub type CMP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READY` reader - READY
pub type READY_R = crate::BitReader;
impl R {
    ///Bit 0 - Compensation cell power-down
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - READY
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPCR")
            .field("ready", &self.ready())
            .field("cmp_pd", &self.cmp_pd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compensation cell power-down
    #[inline(always)]
    pub fn cmp_pd(&mut self) -> CMP_PD_W<'_, CMPCRrs> {
        CMP_PD_W::new(self, 0)
    }
}
/**Compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`cmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#SYSCFG:CMPCR)*/
pub struct CMPCRrs;
impl crate::RegisterSpec for CMPCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmpcr::R`](R) reader structure
impl crate::Readable for CMPCRrs {}
///`write(|w| ..)` method takes [`cmpcr::W`](W) writer structure
impl crate::Writable for CMPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMPCR to value 0
impl crate::Resettable for CMPCRrs {}
