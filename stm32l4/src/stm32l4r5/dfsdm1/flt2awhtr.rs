///Register `FLT2AWHTR` reader
pub type R = crate::R<FLT2AWHTRrs>;
///Register `FLT2AWHTR` writer
pub type W = crate::W<FLT2AWHTRrs>;
///Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event
pub type BKAWH_R = crate::FieldReader;
///Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event
pub type BKAWH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWHT` reader - Analog watchdog high threshold
pub type AWHT_R = crate::FieldReader<u32>;
///Field `AWHT` writer - Analog watchdog high threshold
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - Analog watchdog high threshold
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT2AWHTR")
            .field("awht", &self.awht())
            .field("bkawh", &self.bkawh())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh(&mut self) -> BKAWH_W<FLT2AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    ///Bits 8:31 - Analog watchdog high threshold
    #[inline(always)]
    pub fn awht(&mut self) -> AWHT_W<FLT2AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
/**analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt2awhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2awhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#DFSDM1:FLT2AWHTR)*/
pub struct FLT2AWHTRrs;
impl crate::RegisterSpec for FLT2AWHTRrs {
    type Ux = u32;
}
///`read()` method returns [`flt2awhtr::R`](R) reader structure
impl crate::Readable for FLT2AWHTRrs {}
///`write(|w| ..)` method takes [`flt2awhtr::W`](W) writer structure
impl crate::Writable for FLT2AWHTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT2AWHTR to value 0
impl crate::Resettable for FLT2AWHTRrs {}
