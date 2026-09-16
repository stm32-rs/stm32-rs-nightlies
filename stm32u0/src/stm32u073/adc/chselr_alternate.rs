///Register `CHSELR_ALTERNATE` reader
pub type R = crate::R<CHSELR_ALTERNATErs>;
///Register `CHSELR_ALTERNATE` writer
pub type W = crate::W<CHSELR_ALTERNATErs>;
///Field `SQ1` reader - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ1_R = crate::FieldReader;
///Field `SQ1` writer - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ2` reader - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ2_R = crate::FieldReader;
///Field `SQ2` writer - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ3` reader - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ3_R = crate::FieldReader;
///Field `SQ3` writer - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ4` reader - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ4_R = crate::FieldReader;
///Field `SQ4` writer - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ5` reader - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ5_R = crate::FieldReader;
///Field `SQ5` writer - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ6` reader - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ6_R = crate::FieldReader;
///Field `SQ6` writer - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ7` reader - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ7_R = crate::FieldReader;
///Field `SQ7` writer - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ8` reader - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ8_R = crate::FieldReader;
///Field `SQ8` writer - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR_ALTERNATE")
            .field("sq1", &self.sq1())
            .field("sq2", &self.sq2())
            .field("sq3", &self.sq3())
            .field("sq4", &self.sq4())
            .field("sq5", &self.sq5())
            .field("sq6", &self.sq6())
            .field("sq7", &self.sq7())
            .field("sq8", &self.sq8())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W<'_, CHSELR_ALTERNATErs> {
        SQ1_W::new(self, 0)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W<'_, CHSELR_ALTERNATErs> {
        SQ2_W::new(self, 4)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W<'_, CHSELR_ALTERNATErs> {
        SQ3_W::new(self, 8)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W<'_, CHSELR_ALTERNATErs> {
        SQ4_W::new(self, 12)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<'_, CHSELR_ALTERNATErs> {
        SQ5_W::new(self, 16)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<'_, CHSELR_ALTERNATErs> {
        SQ6_W::new(self, 20)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<'_, CHSELR_ALTERNATErs> {
        SQ7_W::new(self, 24)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<'_, CHSELR_ALTERNATErs> {
        SQ8_W::new(self, 28)
    }
}
/**ADC channel selection register

You can [`read`](crate::Reg::read) this register and get [`chselr_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#ADC:CHSELR_ALTERNATE)*/
pub struct CHSELR_ALTERNATErs;
impl crate::RegisterSpec for CHSELR_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`chselr_alternate::R`](R) reader structure
impl crate::Readable for CHSELR_ALTERNATErs {}
///`write(|w| ..)` method takes [`chselr_alternate::W`](W) writer structure
impl crate::Writable for CHSELR_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CHSELR_ALTERNATE to value 0
impl crate::Resettable for CHSELR_ALTERNATErs {}
