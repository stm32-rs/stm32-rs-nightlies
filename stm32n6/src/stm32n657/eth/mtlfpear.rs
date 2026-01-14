///Register `MTLFPEAR` reader
pub type R = crate::R<MTLFPEARrs>;
///Register `MTLFPEAR` writer
pub type W = crate::W<MTLFPEARrs>;
///Field `HADV` reader - Hold Advance
pub type HADV_R = crate::FieldReader<u16>;
///Field `HADV` writer - Hold Advance
pub type HADV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RADV` reader - Release Advance
pub type RADV_R = crate::FieldReader<u16>;
///Field `RADV` writer - Release Advance
pub type RADV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Hold Advance
    #[inline(always)]
    pub fn hadv(&self) -> HADV_R {
        HADV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Release Advance
    #[inline(always)]
    pub fn radv(&self) -> RADV_R {
        RADV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLFPEAR")
            .field("hadv", &self.hadv())
            .field("radv", &self.radv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Hold Advance
    #[inline(always)]
    pub fn hadv(&mut self) -> HADV_W<'_, MTLFPEARrs> {
        HADV_W::new(self, 0)
    }
    ///Bits 16:31 - Release Advance
    #[inline(always)]
    pub fn radv(&mut self) -> RADV_W<'_, MTLFPEARrs> {
        RADV_W::new(self, 16)
    }
}
/**FPE Frame Preemption Advance Register

You can [`read`](crate::Reg::read) this register and get [`mtlfpear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlfpear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLFPEAR)*/
pub struct MTLFPEARrs;
impl crate::RegisterSpec for MTLFPEARrs {
    type Ux = u32;
}
///`read()` method returns [`mtlfpear::R`](R) reader structure
impl crate::Readable for MTLFPEARrs {}
///`write(|w| ..)` method takes [`mtlfpear::W`](W) writer structure
impl crate::Writable for MTLFPEARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLFPEAR to value 0
impl crate::Resettable for MTLFPEARrs {}
