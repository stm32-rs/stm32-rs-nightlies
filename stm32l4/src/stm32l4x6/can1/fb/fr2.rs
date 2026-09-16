///Register `FR2` reader
pub type R = crate::R<FR2rs>;
///Register `FR2` writer
pub type W = crate::W<FR2rs>;
///Field `FB` reader - Filter bits
pub type FB_R = crate::FieldReader<u32>;
///Field `FB` writer - Filter bits
pub type FB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FR2").field("fb", &self.fb()).finish()
    }
}
impl W {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, FR2rs> {
        FB_W::new(self, 0)
    }
}
/**Filter bank x register 2

You can [`read`](crate::Reg::read) this register and get [`fr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FR2rs;
impl crate::RegisterSpec for FR2rs {
    type Ux = u32;
}
///`read()` method returns [`fr2::R`](R) reader structure
impl crate::Readable for FR2rs {}
///`write(|w| ..)` method takes [`fr2::W`](W) writer structure
impl crate::Writable for FR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FR2 to value 0
impl crate::Resettable for FR2rs {}
