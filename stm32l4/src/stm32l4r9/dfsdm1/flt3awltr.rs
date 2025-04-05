///Register `FLT3AWLTR` reader
pub type R = crate::R<FLT3AWLTRrs>;
///Register `FLT3AWLTR` writer
pub type W = crate::W<FLT3AWLTRrs>;
///Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_R = crate::FieldReader;
///Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWLT` reader - Analog watchdog low threshold
pub type AWLT_R = crate::FieldReader<u32>;
///Field `AWLT` writer - Analog watchdog low threshold
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT3AWLTR")
            .field("awlt", &self.awlt())
            .field("bkawl", &self.bkawl())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W<FLT3AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W<FLT3AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
/**analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt3awltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3awltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:FLT3AWLTR)*/
pub struct FLT3AWLTRrs;
impl crate::RegisterSpec for FLT3AWLTRrs {
    type Ux = u32;
}
///`read()` method returns [`flt3awltr::R`](R) reader structure
impl crate::Readable for FLT3AWLTRrs {}
///`write(|w| ..)` method takes [`flt3awltr::W`](W) writer structure
impl crate::Writable for FLT3AWLTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT3AWLTR to value 0
impl crate::Resettable for FLT3AWLTRrs {}
