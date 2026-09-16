///Register `M1AR` reader
pub type R = crate::R<M1ARrs>;
///Register `M1AR` writer
pub type W = crate::W<M1ARrs>;
///Field `MA` reader - peripheral address
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - peripheral address
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M1AR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<'_, M1ARrs> {
        MA_W::new(self, 0)
    }
}
/**channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`m1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct M1ARrs;
impl crate::RegisterSpec for M1ARrs {
    type Ux = u32;
}
///`read()` method returns [`m1ar::R`](R) reader structure
impl crate::Readable for M1ARrs {}
///`write(|w| ..)` method takes [`m1ar::W`](W) writer structure
impl crate::Writable for M1ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M1AR to value 0
impl crate::Resettable for M1ARrs {}
