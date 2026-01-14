///Register `FLT3FCR` reader
pub type R = crate::R<FLT3FCRrs>;
///Register `FLT3FCR` writer
pub type W = crate::W<FLT3FCRrs>;
///Field `IOSR` reader - IOSR
pub type IOSR_R = crate::FieldReader;
///Field `IOSR` writer - IOSR
pub type IOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FOSR` reader - FOSR
pub type FOSR_R = crate::FieldReader<u16>;
///Field `FOSR` writer - FOSR
pub type FOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `FORD` reader - FORD
pub type FORD_R = crate::FieldReader;
///Field `FORD` writer - FORD
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:7 - IOSR
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - FOSR
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 29:31 - FORD
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT3FCR")
            .field("iosr", &self.iosr())
            .field("fosr", &self.fosr())
            .field("ford", &self.ford())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - IOSR
    #[inline(always)]
    pub fn iosr(&mut self) -> IOSR_W<'_, FLT3FCRrs> {
        IOSR_W::new(self, 0)
    }
    ///Bits 16:25 - FOSR
    #[inline(always)]
    pub fn fosr(&mut self) -> FOSR_W<'_, FLT3FCRrs> {
        FOSR_W::new(self, 16)
    }
    ///Bits 29:31 - FORD
    #[inline(always)]
    pub fn ford(&mut self) -> FORD_W<'_, FLT3FCRrs> {
        FORD_W::new(self, 29)
    }
}
/**DFSDM filter 3 control register

You can [`read`](crate::Reg::read) this register and get [`flt3fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3FCR)*/
pub struct FLT3FCRrs;
impl crate::RegisterSpec for FLT3FCRrs {
    type Ux = u32;
}
///`read()` method returns [`flt3fcr::R`](R) reader structure
impl crate::Readable for FLT3FCRrs {}
///`write(|w| ..)` method takes [`flt3fcr::W`](W) writer structure
impl crate::Writable for FLT3FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT3FCR to value 0
impl crate::Resettable for FLT3FCRrs {}
