///Register `FLT1FCR` reader
pub type R = crate::R<FLT1FCRrs>;
///Register `FLT1FCR` writer
pub type W = crate::W<FLT1FCRrs>;
///Field `IOSR` reader - Integrator oversampling ratio (averaging length)
pub type IOSR_R = crate::FieldReader;
///Field `IOSR` writer - Integrator oversampling ratio (averaging length)
pub type IOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)
pub type FOSR_R = crate::FieldReader<u16>;
///Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)
pub type FOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `FORD` reader - Sinc filter order
pub type FORD_R = crate::FieldReader;
///Field `FORD` writer - Sinc filter order
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT1FCR")
            .field("ford", &self.ford())
            .field("fosr", &self.fosr())
            .field("iosr", &self.iosr())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    pub fn iosr(&mut self) -> IOSR_W<FLT1FCRrs> {
        IOSR_W::new(self, 0)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    pub fn fosr(&mut self) -> FOSR_W<FLT1FCRrs> {
        FOSR_W::new(self, 16)
    }
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    pub fn ford(&mut self) -> FORD_W<FLT1FCRrs> {
        FORD_W::new(self, 29)
    }
}
/**filter control register

You can [`read`](crate::Reg::read) this register and get [`flt1fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:FLT1FCR)*/
pub struct FLT1FCRrs;
impl crate::RegisterSpec for FLT1FCRrs {
    type Ux = u32;
}
///`read()` method returns [`flt1fcr::R`](R) reader structure
impl crate::Readable for FLT1FCRrs {}
///`write(|w| ..)` method takes [`flt1fcr::W`](W) writer structure
impl crate::Writable for FLT1FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT1FCR to value 0
impl crate::Resettable for FLT1FCRrs {}
