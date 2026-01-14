///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `GLOCK` reader - global lock
pub type GLOCK_R = crate::BitReader;
///Field `GLOCK` writer - global lock
pub type GLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - global lock
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR").field("glock", &self.glock()).finish()
    }
}
impl W {
    ///Bit 0 - global lock
    #[inline(always)]
    pub fn glock(&mut self) -> GLOCK_W<'_, CRrs> {
        GLOCK_W::new(self, 0)
    }
}
/**RISAF configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RISAF:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
