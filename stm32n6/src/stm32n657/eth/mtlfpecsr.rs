///Register `MTLFPECSR` reader
pub type R = crate::R<MTLFPECSRrs>;
///Register `MTLFPECSR` writer
pub type W = crate::W<MTLFPECSRrs>;
///Field `AFSZ` reader - Additional Fragment Size
pub type AFSZ_R = crate::FieldReader;
///Field `AFSZ` writer - Additional Fragment Size
pub type AFSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PEC` reader - Preemption Classification
pub type PEC_R = crate::FieldReader;
///Field `PEC` writer - Preemption Classification
pub type PEC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HRS` reader - Hold/Release Status
pub type HRS_R = crate::BitReader;
///Field `HRS` writer - Hold/Release Status
pub type HRS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Additional Fragment Size
    #[inline(always)]
    pub fn afsz(&self) -> AFSZ_R {
        AFSZ_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - Preemption Classification
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 28 - Hold/Release Status
    #[inline(always)]
    pub fn hrs(&self) -> HRS_R {
        HRS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLFPECSR")
            .field("afsz", &self.afsz())
            .field("pec", &self.pec())
            .field("hrs", &self.hrs())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Additional Fragment Size
    #[inline(always)]
    pub fn afsz(&mut self) -> AFSZ_W<'_, MTLFPECSRrs> {
        AFSZ_W::new(self, 0)
    }
    ///Bits 8:9 - Preemption Classification
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W<'_, MTLFPECSRrs> {
        PEC_W::new(self, 8)
    }
    ///Bit 28 - Hold/Release Status
    #[inline(always)]
    pub fn hrs(&mut self) -> HRS_W<'_, MTLFPECSRrs> {
        HRS_W::new(self, 28)
    }
}
/**FPE Frame Preemption Control Status Register

You can [`read`](crate::Reg::read) this register and get [`mtlfpecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlfpecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLFPECSR)*/
pub struct MTLFPECSRrs;
impl crate::RegisterSpec for MTLFPECSRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlfpecsr::R`](R) reader structure
impl crate::Readable for MTLFPECSRrs {}
///`write(|w| ..)` method takes [`mtlfpecsr::W`](W) writer structure
impl crate::Writable for MTLFPECSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLFPECSR to value 0
impl crate::Resettable for MTLFPECSRrs {}
