///Register `CMR2` reader
pub type R = crate::R<CMR2rs>;
///Register `CMR2` writer
pub type W = crate::W<CMR2rs>;
///Field `PB` reader - Port B channel masking
pub type PB_R = crate::FieldReader<u16>;
///Field `PB` writer - Port B channel masking
pub type PB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port B channel masking
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMR2").field("pb", &self.pb()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port B channel masking
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W<'_, CMR2rs> {
        PB_W::new(self, 0)
    }
}
/**Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RI:CMR2)*/
pub struct CMR2rs;
impl crate::RegisterSpec for CMR2rs {
    type Ux = u32;
}
///`read()` method returns [`cmr2::R`](R) reader structure
impl crate::Readable for CMR2rs {}
///`write(|w| ..)` method takes [`cmr2::W`](W) writer structure
impl crate::Writable for CMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMR2 to value 0
impl crate::Resettable for CMR2rs {}
