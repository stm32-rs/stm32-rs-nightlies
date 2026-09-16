///Register `CONF2R` reader
pub type R = crate::R<CONF2Rrs>;
///Register `CONF2R` writer
pub type W = crate::W<CONF2Rrs>;
///Field `OFFSET2` reader - Twelve-bit calibration offset for configuration 2
pub type OFFSET2_R = crate::FieldReader<u16>;
///Field `OFFSET2` writer - Twelve-bit calibration offset for configuration 2
pub type OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `GAIN2` reader - Gain setting for configuration 2
pub type GAIN2_R = crate::FieldReader;
///Field `GAIN2` writer - Gain setting for configuration 2
pub type GAIN2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SE2` reader - Single-ended mode for configuration 2
pub type SE2_R = crate::FieldReader;
///Field `SE2` writer - Single-ended mode for configuration 2
pub type SE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMMON2` reader - Common mode for configuration 2
pub type COMMON2_R = crate::FieldReader;
///Field `COMMON2` writer - Common mode for configuration 2
pub type COMMON2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:11 - Twelve-bit calibration offset for configuration 2
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 20:22 - Gain setting for configuration 2
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 26:27 - Single-ended mode for configuration 2
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 30:31 - Common mode for configuration 2
    #[inline(always)]
    pub fn common2(&self) -> COMMON2_R {
        COMMON2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF2R")
            .field("common2", &self.common2())
            .field("se2", &self.se2())
            .field("gain2", &self.gain2())
            .field("offset2", &self.offset2())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Twelve-bit calibration offset for configuration 2
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W<'_, CONF2Rrs> {
        OFFSET2_W::new(self, 0)
    }
    ///Bits 20:22 - Gain setting for configuration 2
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W<'_, CONF2Rrs> {
        GAIN2_W::new(self, 20)
    }
    ///Bits 26:27 - Single-ended mode for configuration 2
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W<'_, CONF2Rrs> {
        SE2_W::new(self, 26)
    }
    ///Bits 30:31 - Common mode for configuration 2
    #[inline(always)]
    pub fn common2(&mut self) -> COMMON2_W<'_, CONF2Rrs> {
        COMMON2_W::new(self, 30)
    }
}
/**configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`conf2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONF2R)*/
pub struct CONF2Rrs;
impl crate::RegisterSpec for CONF2Rrs {
    type Ux = u32;
}
///`read()` method returns [`conf2r::R`](R) reader structure
impl crate::Readable for CONF2Rrs {}
///`write(|w| ..)` method takes [`conf2r::W`](W) writer structure
impl crate::Writable for CONF2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONF2R to value 0
impl crate::Resettable for CONF2Rrs {}
