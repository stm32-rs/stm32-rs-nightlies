///Register `FLT4AWLTR` reader
pub type R = crate::R<FLT4AWLTRrs>;
///Register `FLT4AWLTR` writer
pub type W = crate::W<FLT4AWLTRrs>;
///Field `BKAWL` reader - BKAWL
pub type BKAWL_R = crate::FieldReader;
///Field `BKAWL` writer - BKAWL
pub type BKAWL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWLT` reader - AWLT
pub type AWLT_R = crate::FieldReader<u32>;
///Field `AWLT` writer - AWLT
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - BKAWL
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - AWLT
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT4AWLTR")
            .field("bkawl", &self.bkawl())
            .field("awlt", &self.awlt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - BKAWL
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W<FLT4AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    ///Bits 8:31 - AWLT
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W<FLT4AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
/**DFSDM filter 4 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt4awltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4awltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT4AWLTR)*/
pub struct FLT4AWLTRrs;
impl crate::RegisterSpec for FLT4AWLTRrs {
    type Ux = u32;
}
///`read()` method returns [`flt4awltr::R`](R) reader structure
impl crate::Readable for FLT4AWLTRrs {}
///`write(|w| ..)` method takes [`flt4awltr::W`](W) writer structure
impl crate::Writable for FLT4AWLTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLT4AWLTR to value 0
impl crate::Resettable for FLT4AWLTRrs {
    const RESET_VALUE: u32 = 0;
}