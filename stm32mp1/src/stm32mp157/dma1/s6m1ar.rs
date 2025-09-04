///Register `S6M1AR` reader
pub type R = crate::R<S6M1ARrs>;
///Register `S6M1AR` writer
pub type W = crate::W<S6M1ARrs>;
///Field `M1A` reader - M1A
pub type M1A_R = crate::FieldReader<u32>;
///Field `M1A` writer - M1A
pub type M1A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - M1A
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6M1AR").field("m1a", &self.m1a()).finish()
    }
}
impl W {
    ///Bits 0:31 - M1A
    #[inline(always)]
    pub fn m1a(&mut self) -> M1A_W<S6M1ARrs> {
        M1A_W::new(self, 0)
    }
}
/**DMA stream 6 memory 1 address register

You can [`read`](crate::Reg::read) this register and get [`s6m1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6m1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMA1:S6M1AR)*/
pub struct S6M1ARrs;
impl crate::RegisterSpec for S6M1ARrs {
    type Ux = u32;
}
///`read()` method returns [`s6m1ar::R`](R) reader structure
impl crate::Readable for S6M1ARrs {}
///`write(|w| ..)` method takes [`s6m1ar::W`](W) writer structure
impl crate::Writable for S6M1ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S6M1AR to value 0
impl crate::Resettable for S6M1ARrs {}
