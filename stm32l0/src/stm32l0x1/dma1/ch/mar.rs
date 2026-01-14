///Register `MAR` reader
pub type R = crate::R<MARrs>;
///Register `MAR` writer
pub type W = crate::W<MARrs>;
///Field `MA` reader - Memory address
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Memory address
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<'_, MARrs> {
        MA_W::new(self, 0)
    }
}
/**channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MARrs;
impl crate::RegisterSpec for MARrs {
    type Ux = u32;
}
///`read()` method returns [`mar::R`](R) reader structure
impl crate::Readable for MARrs {}
///`write(|w| ..)` method takes [`mar::W`](W) writer structure
impl crate::Writable for MARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAR to value 0
impl crate::Resettable for MARrs {}
