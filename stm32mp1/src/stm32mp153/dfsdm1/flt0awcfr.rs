///Register `FLT0AWCFR` reader
pub type R = crate::R<FLT0AWCFRrs>;
///Register `FLT0AWCFR` writer
pub type W = crate::W<FLT0AWCFRrs>;
///Field `CLRAWLTF` reader - CLRAWLTF
pub type CLRAWLTF_R = crate::FieldReader;
///Field `CLRAWLTF` writer - CLRAWLTF
pub type CLRAWLTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLRAWHTF` reader - CLRAWHTF
pub type CLRAWHTF_R = crate::FieldReader;
///Field `CLRAWHTF` writer - CLRAWHTF
pub type CLRAWHTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - CLRAWLTF
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - CLRAWHTF
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT0AWCFR")
            .field("clrawltf", &self.clrawltf())
            .field("clrawhtf", &self.clrawhtf())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - CLRAWLTF
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<'_, FLT0AWCFRrs> {
        CLRAWLTF_W::new(self, 0)
    }
    ///Bits 8:15 - CLRAWHTF
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W<'_, FLT0AWCFRrs> {
        CLRAWHTF_W::new(self, 8)
    }
}
/**DFSDM filter 0 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt0awcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0awcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT0AWCFR)*/
pub struct FLT0AWCFRrs;
impl crate::RegisterSpec for FLT0AWCFRrs {
    type Ux = u32;
}
///`read()` method returns [`flt0awcfr::R`](R) reader structure
impl crate::Readable for FLT0AWCFRrs {}
///`write(|w| ..)` method takes [`flt0awcfr::W`](W) writer structure
impl crate::Writable for FLT0AWCFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT0AWCFR to value 0
impl crate::Resettable for FLT0AWCFRrs {}
