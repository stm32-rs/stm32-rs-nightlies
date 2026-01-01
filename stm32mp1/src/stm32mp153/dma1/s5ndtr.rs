///Register `S5NDTR` reader
pub type R = crate::R<S5NDTRrs>;
///Register `S5NDTR` writer
pub type W = crate::W<S5NDTRrs>;
///Field `NDT` reader - NDT
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - NDT
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NDT
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S5NDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - NDT
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<'_, S5NDTRrs> {
        NDT_W::new(self, 0)
    }
}
/**DMA stream 5 number of data register

You can [`read`](crate::Reg::read) this register and get [`s5ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S5NDTR)*/
pub struct S5NDTRrs;
impl crate::RegisterSpec for S5NDTRrs {
    type Ux = u32;
}
///`read()` method returns [`s5ndtr::R`](R) reader structure
impl crate::Readable for S5NDTRrs {}
///`write(|w| ..)` method takes [`s5ndtr::W`](W) writer structure
impl crate::Writable for S5NDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S5NDTR to value 0
impl crate::Resettable for S5NDTRrs {}
