///Register `FR1` reader
pub type R = crate::R<FR1rs>;
///Register `FR1` writer
pub type W = crate::W<FR1rs>;
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
        f.debug_struct("FR1").field("fb", &self.fb()).finish()
    }
}
impl W {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, FR1rs> {
        FB_W::new(self, 0)
    }
}
/**Filter bank x register 1

You can [`read`](crate::Reg::read) this register and get [`fr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FR1rs;
impl crate::RegisterSpec for FR1rs {
    type Ux = u32;
}
///`read()` method returns [`fr1::R`](R) reader structure
impl crate::Readable for FR1rs {}
///`write(|w| ..)` method takes [`fr1::W`](W) writer structure
impl crate::Writable for FR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FR1 to value 0
impl crate::Resettable for FR1rs {}
