///Register `S2M0AR` reader
pub type R = crate::R<S2M0ARrs>;
///Register `S2M0AR` writer
pub type W = crate::W<S2M0ARrs>;
///Field `M0A` reader - M0A
pub type M0A_R = crate::FieldReader<u32>;
///Field `M0A` writer - M0A
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - M0A
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2M0AR").field("m0a", &self.m0a()).finish()
    }
}
impl W {
    ///Bits 0:31 - M0A
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<'_, S2M0ARrs> {
        M0A_W::new(self, 0)
    }
}
/**DMA stream 2 memory 0 address register

You can [`read`](crate::Reg::read) this register and get [`s2m0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2m0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S2M0AR)*/
pub struct S2M0ARrs;
impl crate::RegisterSpec for S2M0ARrs {
    type Ux = u32;
}
///`read()` method returns [`s2m0ar::R`](R) reader structure
impl crate::Readable for S2M0ARrs {}
///`write(|w| ..)` method takes [`s2m0ar::W`](W) writer structure
impl crate::Writable for S2M0ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S2M0AR to value 0
impl crate::Resettable for S2M0ARrs {}
