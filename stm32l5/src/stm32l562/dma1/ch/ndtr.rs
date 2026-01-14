///Register `NDTR` reader
pub type R = crate::R<NDTRrs>;
///Register `NDTR` writer
pub type W = crate::W<NDTRrs>;
///Field `NDT` reader - Number of data to transfer
pub type NDT_R = crate::FieldReader<u32>;
///Field `NDT` writer - Number of data to transfer
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 0:17 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:17 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<'_, NDTRrs> {
        NDT_W::new(self, 0)
    }
}
/**channel x number of data register

You can [`read`](crate::Reg::read) this register and get [`ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NDTRrs;
impl crate::RegisterSpec for NDTRrs {
    type Ux = u32;
}
///`read()` method returns [`ndtr::R`](R) reader structure
impl crate::Readable for NDTRrs {}
///`write(|w| ..)` method takes [`ndtr::W`](W) writer structure
impl crate::Writable for NDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NDTR to value 0
impl crate::Resettable for NDTRrs {}
