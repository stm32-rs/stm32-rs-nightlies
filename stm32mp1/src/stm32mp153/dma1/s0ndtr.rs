///Register `S0NDTR` reader
pub type R = crate::R<S0NDTRrs>;
///Register `S0NDTR` writer
pub type W = crate::W<S0NDTRrs>;
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
        f.debug_struct("S0NDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - NDT
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<S0NDTRrs> {
        NDT_W::new(self, 0)
    }
}
/**DMA stream 0 number of data register

You can [`read`](crate::Reg::read) this register and get [`s0ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S0NDTR)*/
pub struct S0NDTRrs;
impl crate::RegisterSpec for S0NDTRrs {
    type Ux = u32;
}
///`read()` method returns [`s0ndtr::R`](R) reader structure
impl crate::Readable for S0NDTRrs {}
///`write(|w| ..)` method takes [`s0ndtr::W`](W) writer structure
impl crate::Writable for S0NDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S0NDTR to value 0
impl crate::Resettable for S0NDTRrs {}
