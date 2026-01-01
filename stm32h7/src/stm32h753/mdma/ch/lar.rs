///Register `LAR` reader
pub type R = crate::R<LARrs>;
///Register `LAR` writer
pub type W = crate::W<LARrs>;
///Field `LAR` reader - Link address register
pub type LAR_R = crate::FieldReader<u32>;
///Field `LAR` writer - Link address register
pub type LAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Link address register
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAR").field("lar", &self.lar()).finish()
    }
}
impl W {
    ///Bits 0:31 - Link address register
    #[inline(always)]
    pub fn lar(&mut self) -> LAR_W<'_, LARrs> {
        LAR_W::new(self, 0)
    }
}
/**MDMA channel x Link Address register

You can [`read`](crate::Reg::read) this register and get [`lar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LARrs;
impl crate::RegisterSpec for LARrs {
    type Ux = u32;
}
///`read()` method returns [`lar::R`](R) reader structure
impl crate::Readable for LARrs {}
///`write(|w| ..)` method takes [`lar::W`](W) writer structure
impl crate::Writable for LARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LAR to value 0
impl crate::Resettable for LARrs {}
