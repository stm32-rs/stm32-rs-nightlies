///Register `MAR` reader
pub type R = crate::R<MARrs>;
///Register `MAR` writer
pub type W = crate::W<MARrs>;
///Field `MAR` reader - Mask address
pub type MAR_R = crate::FieldReader<u32>;
///Field `MAR` writer - Mask address
pub type MAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Mask address
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAR").field("mar", &self.mar()).finish()
    }
}
impl W {
    ///Bits 0:31 - Mask address
    #[inline(always)]
    pub fn mar(&mut self) -> MAR_W<'_, MARrs> {
        MAR_W::new(self, 0)
    }
}
/**MDMA channel x Mask address register

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
