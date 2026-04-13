///Register `ADC_CHSELR_ALTERNATE1` reader
pub type R = crate::R<ADC_CHSELR_ALTERNATE1rs>;
///Register `ADC_CHSELR_ALTERNATE1` writer
pub type W = crate::W<ADC_CHSELR_ALTERNATE1rs>;
///Field `SQ1` reader - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ1_R = crate::FieldReader;
///Field `SQ1` writer - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ2` reader - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ2_R = crate::FieldReader;
///Field `SQ2` writer - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ3` reader - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ3_R = crate::FieldReader;
///Field `SQ3` writer - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ4` reader - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ4_R = crate::FieldReader;
///Field `SQ4` writer - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ5` reader - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ5_R = crate::FieldReader;
///Field `SQ5` writer - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ6` reader - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ6_R = crate::FieldReader;
///Field `SQ6` writer - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ7` reader - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ7_R = crate::FieldReader;
///Field `SQ7` writer - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQ8 {
    ///0: CH0
    B0x0 = 0,
    ///1: CH1
    B0x1 = 1,
    ///12: CH12
    B0xC = 12,
    ///13: CH13
    B0xD = 13,
    ///14: CH14
    B0xE = 14,
    ///15: No channel selected (End of sequence)
    B0xF = 15,
}
impl From<SQ8> for u8 {
    #[inline(always)]
    fn from(variant: SQ8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SQ8 {
    type Ux = u8;
}
impl crate::IsEnum for SQ8 {}
///Field `SQ8` reader - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ8_R = crate::FieldReader<SQ8>;
impl SQ8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SQ8> {
        match self.bits {
            0 => Some(SQ8::B0x0),
            1 => Some(SQ8::B0x1),
            12 => Some(SQ8::B0xC),
            13 => Some(SQ8::B0xD),
            14 => Some(SQ8::B0xE),
            15 => Some(SQ8::B0xF),
            _ => None,
        }
    }
    ///CH0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SQ8::B0x0
    }
    ///CH1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SQ8::B0x1
    }
    ///CH12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == SQ8::B0xC
    }
    ///CH13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == SQ8::B0xD
    }
    ///CH14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == SQ8::B0xE
    }
    ///No channel selected (End of sequence)
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == SQ8::B0xF
    }
}
///Field `SQ8` writer - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SQ8>;
impl<'a, REG> SQ8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CH0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SQ8::B0x0)
    }
    ///CH1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SQ8::B0x1)
    }
    ///CH12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(SQ8::B0xC)
    }
    ///CH13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(SQ8::B0xD)
    }
    ///CH14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(SQ8::B0xE)
    }
    ///No channel selected (End of sequence)
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(SQ8::B0xF)
    }
}
impl R {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CHSELR_ALTERNATE1")
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
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ1_W::new(self, 0)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ2_W::new(self, 4)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ3_W::new(self, 8)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ4_W::new(self, 12)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ5_W::new(self, 16)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ6_W::new(self, 20)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ7_W::new(self, 24)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<'_, ADC_CHSELR_ALTERNATE1rs> {
        SQ8_W::new(self, 28)
    }
}
/**ADC channel selection register

You can [`read`](crate::Reg::read) this register and get [`adc_chselr_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chselr_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#ADC:ADC_CHSELR_ALTERNATE1)*/
pub struct ADC_CHSELR_ALTERNATE1rs;
impl crate::RegisterSpec for ADC_CHSELR_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_chselr_alternate1::R`](R) reader structure
impl crate::Readable for ADC_CHSELR_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`adc_chselr_alternate1::W`](W) writer structure
impl crate::Writable for ADC_CHSELR_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CHSELR_ALTERNATE1 to value 0
impl crate::Resettable for ADC_CHSELR_ALTERNATE1rs {}
