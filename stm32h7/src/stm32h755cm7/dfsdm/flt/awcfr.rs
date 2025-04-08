///Register `AWCFR` reader
pub type R = crate::R<AWCFRrs>;
///Register `AWCFR` writer
pub type W = crate::W<AWCFRrs>;
///Field `CLRAWLTF` reader - Clear the analog watchdog low threshold flag
pub type CLRAWLTF_R = crate::FieldReader;
///Field `CLRAWLTF` writer - Clear the analog watchdog low threshold flag
pub type CLRAWLTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLRAWHTF` reader - Clear the analog watchdog high threshold flag
pub type CLRAWHTF_R = crate::FieldReader;
///Field `CLRAWHTF` writer - Clear the analog watchdog high threshold flag
pub type CLRAWHTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWCFR")
            .field("clrawhtf", &self.clrawhtf())
            .field("clrawltf", &self.clrawltf())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<AWCFRrs> {
        CLRAWLTF_W::new(self, 0)
    }
    ///Bits 8:15 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W<AWCFRrs> {
        CLRAWHTF_W::new(self, 8)
    }
}
/**analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`awcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWCFRrs;
impl crate::RegisterSpec for AWCFRrs {
    type Ux = u32;
}
///`read()` method returns [`awcfr::R`](R) reader structure
impl crate::Readable for AWCFRrs {}
///`write(|w| ..)` method takes [`awcfr::W`](W) writer structure
impl crate::Writable for AWCFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWCFR to value 0
impl crate::Resettable for AWCFRrs {}
