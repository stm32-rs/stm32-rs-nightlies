///Register `CONF1R` reader
pub type R = crate::R<CONF1Rrs>;
///Register `CONF1R` writer
pub type W = crate::W<CONF1Rrs>;
///Field `OFFSET1` reader - Twelve-bit calibration offset for configuration 1
pub type OFFSET1_R = crate::FieldReader<u16>;
///Field `OFFSET1` writer - Twelve-bit calibration offset for configuration 1
pub type OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `GAIN1` reader - Gain setting for configuration 1
pub type GAIN1_R = crate::FieldReader;
///Field `GAIN1` writer - Gain setting for configuration 1
pub type GAIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SE1` reader - Single-ended mode for configuration 1
pub type SE1_R = crate::FieldReader;
///Field `SE1` writer - Single-ended mode for configuration 1
pub type SE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMMON1` reader - Common mode for configuration 1
pub type COMMON1_R = crate::FieldReader;
///Field `COMMON1` writer - Common mode for configuration 1
pub type COMMON1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:11 - Twelve-bit calibration offset for configuration 1
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 20:22 - Gain setting for configuration 1
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 26:27 - Single-ended mode for configuration 1
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 30:31 - Common mode for configuration 1
    #[inline(always)]
    pub fn common1(&self) -> COMMON1_R {
        COMMON1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1R")
            .field("common1", &self.common1())
            .field("se1", &self.se1())
            .field("gain1", &self.gain1())
            .field("offset1", &self.offset1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Twelve-bit calibration offset for configuration 1
    #[inline(always)]
    pub fn offset1(&mut self) -> OFFSET1_W<'_, CONF1Rrs> {
        OFFSET1_W::new(self, 0)
    }
    ///Bits 20:22 - Gain setting for configuration 1
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W<'_, CONF1Rrs> {
        GAIN1_W::new(self, 20)
    }
    ///Bits 26:27 - Single-ended mode for configuration 1
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W<'_, CONF1Rrs> {
        SE1_W::new(self, 26)
    }
    ///Bits 30:31 - Common mode for configuration 1
    #[inline(always)]
    pub fn common1(&mut self) -> COMMON1_W<'_, CONF1Rrs> {
        COMMON1_W::new(self, 30)
    }
}
/**configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`conf1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONF1R)*/
pub struct CONF1Rrs;
impl crate::RegisterSpec for CONF1Rrs {
    type Ux = u32;
}
///`read()` method returns [`conf1r::R`](R) reader structure
impl crate::Readable for CONF1Rrs {}
///`write(|w| ..)` method takes [`conf1r::W`](W) writer structure
impl crate::Writable for CONF1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONF1R to value 0
impl crate::Resettable for CONF1Rrs {}
