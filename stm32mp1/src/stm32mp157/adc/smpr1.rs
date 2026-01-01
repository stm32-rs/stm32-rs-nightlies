///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
///Field `SMP0` reader - SMP0
pub type SMP0_R = crate::FieldReader;
///Field `SMP0` writer - SMP0
pub type SMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP1` reader - SMP1
pub type SMP1_R = crate::FieldReader;
///Field `SMP1` writer - SMP1
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP2` reader - SMP2
pub type SMP2_R = crate::FieldReader;
///Field `SMP2` writer - SMP2
pub type SMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP3` reader - SMP3
pub type SMP3_R = crate::FieldReader;
///Field `SMP3` writer - SMP3
pub type SMP3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP4` reader - SMP4
pub type SMP4_R = crate::FieldReader;
///Field `SMP4` writer - SMP4
pub type SMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP5` reader - SMP5
pub type SMP5_R = crate::FieldReader;
///Field `SMP5` writer - SMP5
pub type SMP5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP6` reader - SMP6
pub type SMP6_R = crate::FieldReader;
///Field `SMP6` writer - SMP6
pub type SMP6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP7` reader - SMP7
pub type SMP7_R = crate::FieldReader;
///Field `SMP7` writer - SMP7
pub type SMP7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP8` reader - SMP8
pub type SMP8_R = crate::FieldReader;
///Field `SMP8` writer - SMP8
pub type SMP8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP9` reader - SMP9
pub type SMP9_R = crate::FieldReader;
///Field `SMP9` writer - SMP9
pub type SMP9_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SMP0
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SMP1
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SMP2
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SMP3
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SMP4
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SMP5
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SMP6
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SMP7
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SMP8
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - SMP9
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp0", &self.smp0())
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smp3", &self.smp3())
            .field("smp4", &self.smp4())
            .field("smp5", &self.smp5())
            .field("smp6", &self.smp6())
            .field("smp7", &self.smp7())
            .field("smp8", &self.smp8())
            .field("smp9", &self.smp9())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMP0
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP0_W<'_, SMPR1rs> {
        SMP0_W::new(self, 0)
    }
    ///Bits 3:5 - SMP1
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<'_, SMPR1rs> {
        SMP1_W::new(self, 3)
    }
    ///Bits 6:8 - SMP2
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<'_, SMPR1rs> {
        SMP2_W::new(self, 6)
    }
    ///Bits 9:11 - SMP3
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W<'_, SMPR1rs> {
        SMP3_W::new(self, 9)
    }
    ///Bits 12:14 - SMP4
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W<'_, SMPR1rs> {
        SMP4_W::new(self, 12)
    }
    ///Bits 15:17 - SMP5
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W<'_, SMPR1rs> {
        SMP5_W::new(self, 15)
    }
    ///Bits 18:20 - SMP6
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W<'_, SMPR1rs> {
        SMP6_W::new(self, 18)
    }
    ///Bits 21:23 - SMP7
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W<'_, SMPR1rs> {
        SMP7_W::new(self, 21)
    }
    ///Bits 24:26 - SMP8
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W<'_, SMPR1rs> {
        SMP8_W::new(self, 24)
    }
    ///Bits 27:29 - SMP9
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W<'_, SMPR1rs> {
        SMP9_W::new(self, 27)
    }
}
/**ADC sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC:SMPR1)*/
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
///`read()` method returns [`smpr1::R`](R) reader structure
impl crate::Readable for SMPR1rs {}
///`write(|w| ..)` method takes [`smpr1::W`](W) writer structure
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1rs {}
