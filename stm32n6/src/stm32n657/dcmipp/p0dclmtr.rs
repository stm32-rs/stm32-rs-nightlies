///Register `P0DCLMTR` reader
pub type R = crate::R<P0DCLMTRrs>;
///Register `P0DCLMTR` writer
pub type W = crate::W<P0DCLMTRrs>;
///Field `LIMIT` reader - Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation.
pub type LIMIT_R = crate::FieldReader<u32>;
///Field `LIMIT` writer - Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation.
pub type LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:23 - Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation.
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0DCLMTR")
            .field("limit", &self.limit())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation.
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W<'_, P0DCLMTRrs> {
        LIMIT_W::new(self, 0)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P0DCLMTRrs> {
        ENABLE_W::new(self, 31)
    }
}
/**DCMIPP Pipe0 dump limit register

You can [`read`](crate::Reg::read) this register and get [`p0dclmtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0dclmtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0DCLMTR)*/
pub struct P0DCLMTRrs;
impl crate::RegisterSpec for P0DCLMTRrs {
    type Ux = u32;
}
///`read()` method returns [`p0dclmtr::R`](R) reader structure
impl crate::Readable for P0DCLMTRrs {}
///`write(|w| ..)` method takes [`p0dclmtr::W`](W) writer structure
impl crate::Writable for P0DCLMTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0DCLMTR to value 0x00ff_ffff
impl crate::Resettable for P0DCLMTRrs {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
