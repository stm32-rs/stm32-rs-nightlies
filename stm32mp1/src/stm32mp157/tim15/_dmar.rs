///Register `_DMAR` reader
pub type R = crate::R<_DMARrs>;
///Register `_DMAR` writer
pub type W = crate::W<_DMARrs>;
///Field `DMAB` reader - DMAB
pub type DMAB_R = crate::FieldReader<u16>;
///Field `DMAB` writer - DMAB
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DMAB
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_DMAR").field("dmab", &self.dmab()).finish()
    }
}
impl W {
    ///Bits 0:15 - DMAB
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W<'_, _DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM15 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_DMAR)*/
pub struct _DMARrs;
impl crate::RegisterSpec for _DMARrs {
    type Ux = u16;
}
///`read()` method returns [`_dmar::R`](R) reader structure
impl crate::Readable for _DMARrs {}
///`write(|w| ..)` method takes [`_dmar::W`](W) writer structure
impl crate::Writable for _DMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _DMAR to value 0
impl crate::Resettable for _DMARrs {}
