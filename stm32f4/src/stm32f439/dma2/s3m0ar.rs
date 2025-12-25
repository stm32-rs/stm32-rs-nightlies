///Register `S3M0AR` reader
pub type R = crate::R<S3M0ARrs>;
///Register `S3M0AR` writer
pub type W = crate::W<S3M0ARrs>;
///Field `M0A` reader - Memory 0 address
pub type M0A_R = crate::FieldReader<u32>;
///Field `M0A` writer - Memory 0 address
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory 0 address
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S3M0AR").field("m0a", &self.m0a()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory 0 address
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<'_, S3M0ARrs> {
        M0A_W::new(self, 0)
    }
}
/**stream x memory 0 address register

You can [`read`](crate::Reg::read) this register and get [`s3m0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3m0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DMA2:S3M0AR)*/
pub struct S3M0ARrs;
impl crate::RegisterSpec for S3M0ARrs {
    type Ux = u32;
}
///`read()` method returns [`s3m0ar::R`](R) reader structure
impl crate::Readable for S3M0ARrs {}
///`write(|w| ..)` method takes [`s3m0ar::W`](W) writer structure
impl crate::Writable for S3M0ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S3M0AR to value 0
impl crate::Resettable for S3M0ARrs {}
