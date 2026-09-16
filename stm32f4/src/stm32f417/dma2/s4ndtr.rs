///Register `S4NDTR` reader
pub type R = crate::R<S4NDTRrs>;
///Register `S4NDTR` writer
pub type W = crate::W<S4NDTRrs>;
///Field `NDT` reader - Number of data items to transfer
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - Number of data items to transfer
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of data items to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S4NDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - Number of data items to transfer
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<'_, S4NDTRrs> {
        NDT_W::new(self, 0)
    }
}
/**stream x number of data register

You can [`read`](crate::Reg::read) this register and get [`s4ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#DMA2:S4NDTR)*/
pub struct S4NDTRrs;
impl crate::RegisterSpec for S4NDTRrs {
    type Ux = u32;
}
///`read()` method returns [`s4ndtr::R`](R) reader structure
impl crate::Readable for S4NDTRrs {}
///`write(|w| ..)` method takes [`s4ndtr::W`](W) writer structure
impl crate::Writable for S4NDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S4NDTR to value 0
impl crate::Resettable for S4NDTRrs {}
