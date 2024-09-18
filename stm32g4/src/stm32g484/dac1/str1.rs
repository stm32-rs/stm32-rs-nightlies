///Register `STR1` reader
pub type R = crate::R<STR1rs>;
///Register `STR1` writer
pub type W = crate::W<STR1rs>;
///Field `STRSTDATA1` reader - DAC Channel 1 Sawtooth reset value
pub type STRSTDATA1_R = crate::FieldReader<u16>;
///Field `STRSTDATA1` writer - DAC Channel 1 Sawtooth reset value
pub type STRSTDATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `STDIR1` reader - DAC Channel1 Sawtooth direction setting
pub type STDIR1_R = crate::BitReader;
///Field `STDIR1` writer - DAC Channel1 Sawtooth direction setting
pub type STDIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STINCDATA1` reader - DAC CH1 Sawtooth increment value (12.4 bit format)
pub type STINCDATA1_R = crate::FieldReader<u16>;
///Field `STINCDATA1` writer - DAC CH1 Sawtooth increment value (12.4 bit format)
pub type STINCDATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:11 - DAC Channel 1 Sawtooth reset value
    #[inline(always)]
    pub fn strstdata1(&self) -> STRSTDATA1_R {
        STRSTDATA1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - DAC Channel1 Sawtooth direction setting
    #[inline(always)]
    pub fn stdir1(&self) -> STDIR1_R {
        STDIR1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)
    #[inline(always)]
    pub fn stincdata1(&self) -> STINCDATA1_R {
        STINCDATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR1")
            .field("strstdata1", &self.strstdata1())
            .field("stdir1", &self.stdir1())
            .field("stincdata1", &self.stincdata1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - DAC Channel 1 Sawtooth reset value
    #[inline(always)]
    #[must_use]
    pub fn strstdata1(&mut self) -> STRSTDATA1_W<STR1rs> {
        STRSTDATA1_W::new(self, 0)
    }
    ///Bit 12 - DAC Channel1 Sawtooth direction setting
    #[inline(always)]
    #[must_use]
    pub fn stdir1(&mut self) -> STDIR1_W<STR1rs> {
        STDIR1_W::new(self, 12)
    }
    ///Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)
    #[inline(always)]
    #[must_use]
    pub fn stincdata1(&mut self) -> STINCDATA1_W<STR1rs> {
        STINCDATA1_W::new(self, 16)
    }
}
/**Sawtooth register

You can [`read`](crate::Reg::read) this register and get [`str1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#DAC1:STR1)*/
pub struct STR1rs;
impl crate::RegisterSpec for STR1rs {
    type Ux = u32;
}
///`read()` method returns [`str1::R`](R) reader structure
impl crate::Readable for STR1rs {}
///`write(|w| ..)` method takes [`str1::W`](W) writer structure
impl crate::Writable for STR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STR1 to value 0
impl crate::Resettable for STR1rs {
    const RESET_VALUE: u32 = 0;
}
