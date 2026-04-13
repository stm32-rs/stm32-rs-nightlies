///Register `FLT3AWCFR` reader
pub type R = crate::R<FLT3AWCFRrs>;
///Register `FLT3AWCFR` writer
pub type W = crate::W<FLT3AWCFRrs>;
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
        f.debug_struct("FLT3AWCFR")
            .field("clrawltf", &self.clrawltf())
            .field("clrawhtf", &self.clrawhtf())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - CLRAWLTF
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<'_, FLT3AWCFRrs> {
        CLRAWLTF_W::new(self, 0)
    }
    ///Bits 8:15 - CLRAWHTF
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W<'_, FLT3AWCFRrs> {
        CLRAWHTF_W::new(self, 8)
    }
}
/**DFSDM filter 3 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt3awcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3awcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3AWCFR)*/
pub struct FLT3AWCFRrs;
impl crate::RegisterSpec for FLT3AWCFRrs {
    type Ux = u32;
}
///`read()` method returns [`flt3awcfr::R`](R) reader structure
impl crate::Readable for FLT3AWCFRrs {}
///`write(|w| ..)` method takes [`flt3awcfr::W`](W) writer structure
impl crate::Writable for FLT3AWCFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT3AWCFR to value 0
impl crate::Resettable for FLT3AWCFRrs {}
