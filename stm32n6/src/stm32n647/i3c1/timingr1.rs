///Register `TIMINGR1` reader
pub type R = crate::R<TIMINGR1rs>;
///Register `TIMINGR1` writer
pub type W = crate::W<TIMINGR1rs>;
///Field `AVAL` reader - Number of kernel clock cycles to set a time unit of 1 s, whatever I3C acts as controller or target.
pub type AVAL_R = crate::FieldReader;
///Field `AVAL` writer - Number of kernel clock cycles to set a time unit of 1 s, whatever I3C acts as controller or target.
pub type AVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ASNCR` reader - Activity state of the new controller (when I3C acts as active controller)
pub type ASNCR_R = crate::FieldReader;
///Field `ASNCR` writer - Activity state of the new controller (when I3C acts as active controller)
pub type ASNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FREE` reader - Number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C acts as controller)
pub type FREE_R = crate::FieldReader;
///Field `FREE` writer - Number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C acts as controller)
pub type FREE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SDA_HD` reader - SDA hold time (when the I3C acts as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tless thansub>HD_PPless than/sub>):
pub type SDA_HD_R = crate::BitReader;
///Field `SDA_HD` writer - SDA hold time (when the I3C acts as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tless thansub>HD_PPless than/sub>):
pub type SDA_HD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Number of kernel clock cycles to set a time unit of 1 s, whatever I3C acts as controller or target.
    #[inline(always)]
    pub fn aval(&self) -> AVAL_R {
        AVAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Activity state of the new controller (when I3C acts as active controller)
    #[inline(always)]
    pub fn asncr(&self) -> ASNCR_R {
        ASNCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:22 - Number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C acts as controller)
    #[inline(always)]
    pub fn free(&self) -> FREE_R {
        FREE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 28 - SDA hold time (when the I3C acts as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tless thansub>HD_PPless than/sub>):
    #[inline(always)]
    pub fn sda_hd(&self) -> SDA_HD_R {
        SDA_HD_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMINGR1")
            .field("aval", &self.aval())
            .field("asncr", &self.asncr())
            .field("free", &self.free())
            .field("sda_hd", &self.sda_hd())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Number of kernel clock cycles to set a time unit of 1 s, whatever I3C acts as controller or target.
    #[inline(always)]
    pub fn aval(&mut self) -> AVAL_W<'_, TIMINGR1rs> {
        AVAL_W::new(self, 0)
    }
    ///Bits 8:9 - Activity state of the new controller (when I3C acts as active controller)
    #[inline(always)]
    pub fn asncr(&mut self) -> ASNCR_W<'_, TIMINGR1rs> {
        ASNCR_W::new(self, 8)
    }
    ///Bits 16:22 - Number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C acts as controller)
    #[inline(always)]
    pub fn free(&mut self) -> FREE_W<'_, TIMINGR1rs> {
        FREE_W::new(self, 16)
    }
    ///Bit 28 - SDA hold time (when the I3C acts as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tless thansub>HD_PPless than/sub>):
    #[inline(always)]
    pub fn sda_hd(&mut self) -> SDA_HD_W<'_, TIMINGR1rs> {
        SDA_HD_W::new(self, 28)
    }
}
/**I3C timing register 1

You can [`read`](crate::Reg::read) this register and get [`timingr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#I3C1:TIMINGR1)*/
pub struct TIMINGR1rs;
impl crate::RegisterSpec for TIMINGR1rs {
    type Ux = u32;
}
///`read()` method returns [`timingr1::R`](R) reader structure
impl crate::Readable for TIMINGR1rs {}
///`write(|w| ..)` method takes [`timingr1::W`](W) writer structure
impl crate::Writable for TIMINGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMINGR1 to value 0
impl crate::Resettable for TIMINGR1rs {}
