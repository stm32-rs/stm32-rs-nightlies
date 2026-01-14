///Register `OFCFGR1` reader
pub type R = crate::R<OFCFGR1rs>;
///Register `OFCFGR1` writer
pub type W = crate::W<OFCFGR1rs>;
///Field `POSOFF` reader - Positive offset enable
pub type POSOFF_R = crate::BitReader;
///Field `POSOFF` writer - Positive offset enable
pub type POSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USAT` reader - Unsigned saturation enable
pub type USAT_R = crate::BitReader;
///Field `USAT` writer - Unsigned saturation enable
pub type USAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSAT` reader - Signed saturation enable
pub type SSAT_R = crate::BitReader;
///Field `SSAT` writer - Signed saturation enable
pub type SSAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET_CH` reader - Channel selection for the data offset y
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - Channel selection for the data offset y
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 24 - Positive offset enable
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Unsigned saturation enable
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Signed saturation enable
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:31 - Channel selection for the data offset y
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFCFGR1")
            .field("posoff", &self.posoff())
            .field("usat", &self.usat())
            .field("ssat", &self.ssat())
            .field("offset_ch", &self.offset_ch())
            .finish()
    }
}
impl W {
    ///Bit 24 - Positive offset enable
    #[inline(always)]
    pub fn posoff(&mut self) -> POSOFF_W<'_, OFCFGR1rs> {
        POSOFF_W::new(self, 24)
    }
    ///Bit 25 - Unsigned saturation enable
    #[inline(always)]
    pub fn usat(&mut self) -> USAT_W<'_, OFCFGR1rs> {
        USAT_W::new(self, 25)
    }
    ///Bit 26 - Signed saturation enable
    #[inline(always)]
    pub fn ssat(&mut self) -> SSAT_W<'_, OFCFGR1rs> {
        SSAT_W::new(self, 26)
    }
    ///Bits 27:31 - Channel selection for the data offset y
    #[inline(always)]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<'_, OFCFGR1rs> {
        OFFSET_CH_W::new(self, 27)
    }
}
/**ADC offset 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`ofcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ADC1:OFCFGR1)*/
pub struct OFCFGR1rs;
impl crate::RegisterSpec for OFCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`ofcfgr1::R`](R) reader structure
impl crate::Readable for OFCFGR1rs {}
///`write(|w| ..)` method takes [`ofcfgr1::W`](W) writer structure
impl crate::Writable for OFCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFCFGR1 to value 0
impl crate::Resettable for OFCFGR1rs {}
