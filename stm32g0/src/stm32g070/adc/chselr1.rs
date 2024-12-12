///Register `CHSELR1` reader
pub type R = crate::R<CHSELR1rs>;
///Register `CHSELR1` writer
pub type W = crate::W<CHSELR1rs>;
/**1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQ1 {
    ///0: Channel 0 selected for the Nth conversion
    Ch0 = 0,
    ///1: Channel 1 selected for the Nth conversion
    Ch1 = 1,
    ///2: Channel 2 selected for the Nth conversion
    Ch2 = 2,
    ///3: Channel 3 selected for the Nth conversion
    Ch3 = 3,
    ///4: Channel 4 selected for the Nth conversion
    Ch4 = 4,
    ///5: Channel 5 selected for the Nth conversion
    Ch5 = 5,
    ///6: Channel 6 selected for the Nth conversion
    Ch6 = 6,
    ///7: Channel 7 selected for the Nth conversion
    Ch7 = 7,
    ///8: Channel 8 selected for the Nth conversion
    Ch8 = 8,
    ///9: Channel 9 selected for the Nth conversion
    Ch9 = 9,
    ///10: Channel 10 selected for the Nth conversion
    Ch10 = 10,
    ///11: Channel 11 selected for the Nth conversion
    Ch11 = 11,
    ///12: Channel 12 selected for the Nth conversion
    Ch12 = 12,
    ///13: Channel 13 selected for the Nth conversion
    Ch13 = 13,
    ///14: Channel 14 selected for the Nth conversion
    Ch14 = 14,
    ///15: End of sequence
    Eos = 15,
}
impl From<SQ1> for u8 {
    #[inline(always)]
    fn from(variant: SQ1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SQ1 {
    type Ux = u8;
}
impl crate::IsEnum for SQ1 {}
/**Field `SQ1` reader - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub type SQ1_R = crate::FieldReader<SQ1>;
impl SQ1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SQ1 {
        match self.bits {
            0 => SQ1::Ch0,
            1 => SQ1::Ch1,
            2 => SQ1::Ch2,
            3 => SQ1::Ch3,
            4 => SQ1::Ch4,
            5 => SQ1::Ch5,
            6 => SQ1::Ch6,
            7 => SQ1::Ch7,
            8 => SQ1::Ch8,
            9 => SQ1::Ch9,
            10 => SQ1::Ch10,
            11 => SQ1::Ch11,
            12 => SQ1::Ch12,
            13 => SQ1::Ch13,
            14 => SQ1::Ch14,
            15 => SQ1::Eos,
            _ => unreachable!(),
        }
    }
    ///Channel 0 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == SQ1::Ch0
    }
    ///Channel 1 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == SQ1::Ch1
    }
    ///Channel 2 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == SQ1::Ch2
    }
    ///Channel 3 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == SQ1::Ch3
    }
    ///Channel 4 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == SQ1::Ch4
    }
    ///Channel 5 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == SQ1::Ch5
    }
    ///Channel 6 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == SQ1::Ch6
    }
    ///Channel 7 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == SQ1::Ch7
    }
    ///Channel 8 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch8(&self) -> bool {
        *self == SQ1::Ch8
    }
    ///Channel 9 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch9(&self) -> bool {
        *self == SQ1::Ch9
    }
    ///Channel 10 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch10(&self) -> bool {
        *self == SQ1::Ch10
    }
    ///Channel 11 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch11(&self) -> bool {
        *self == SQ1::Ch11
    }
    ///Channel 12 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch12(&self) -> bool {
        *self == SQ1::Ch12
    }
    ///Channel 13 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch13(&self) -> bool {
        *self == SQ1::Ch13
    }
    ///Channel 14 selected for the Nth conversion
    #[inline(always)]
    pub fn is_ch14(&self) -> bool {
        *self == SQ1::Ch14
    }
    ///End of sequence
    #[inline(always)]
    pub fn is_eos(&self) -> bool {
        *self == SQ1::Eos
    }
}
/**Field `SQ1` writer - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SQ1, crate::Safe>;
impl<'a, REG> SQ1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Channel 0 selected for the Nth conversion
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch0)
    }
    ///Channel 1 selected for the Nth conversion
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch1)
    }
    ///Channel 2 selected for the Nth conversion
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch2)
    }
    ///Channel 3 selected for the Nth conversion
    #[inline(always)]
    pub fn ch3(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch3)
    }
    ///Channel 4 selected for the Nth conversion
    #[inline(always)]
    pub fn ch4(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch4)
    }
    ///Channel 5 selected for the Nth conversion
    #[inline(always)]
    pub fn ch5(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch5)
    }
    ///Channel 6 selected for the Nth conversion
    #[inline(always)]
    pub fn ch6(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch6)
    }
    ///Channel 7 selected for the Nth conversion
    #[inline(always)]
    pub fn ch7(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch7)
    }
    ///Channel 8 selected for the Nth conversion
    #[inline(always)]
    pub fn ch8(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch8)
    }
    ///Channel 9 selected for the Nth conversion
    #[inline(always)]
    pub fn ch9(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch9)
    }
    ///Channel 10 selected for the Nth conversion
    #[inline(always)]
    pub fn ch10(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch10)
    }
    ///Channel 11 selected for the Nth conversion
    #[inline(always)]
    pub fn ch11(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch11)
    }
    ///Channel 12 selected for the Nth conversion
    #[inline(always)]
    pub fn ch12(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch12)
    }
    ///Channel 13 selected for the Nth conversion
    #[inline(always)]
    pub fn ch13(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch13)
    }
    ///Channel 14 selected for the Nth conversion
    #[inline(always)]
    pub fn ch14(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Ch14)
    }
    ///End of sequence
    #[inline(always)]
    pub fn eos(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Eos)
    }
}
/**Field `SQ2` reader - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_R as SQ2_R;
/**Field `SQ3` reader - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_R as SQ3_R;
/**Field `SQ4` reader - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_R as SQ4_R;
/**Field `SQ5` reader - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_R as SQ5_R;
/**Field `SQ6` reader - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_R as SQ6_R;
/**Field `SQ7` reader - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_R as SQ7_R;
///Field `SQ8` reader - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ8_R;
/**Field `SQ2` writer - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_W as SQ2_W;
/**Field `SQ3` writer - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_W as SQ3_W;
/**Field `SQ4` writer - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_W as SQ4_W;
/**Field `SQ5` writer - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_W as SQ5_W;
/**Field `SQ6` writer - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_W as SQ6_W;
/**Field `SQ7` writer - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub use SQ1_W as SQ7_W;
///Field `SQ8` writer - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ8_W;
impl R {
    /**Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    /**Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    /**Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    /**Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    /**Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    /**Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    /**Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR1")
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
    /**Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W<CHSELR1rs> {
        SQ1_W::new(self, 0)
    }
    /**Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W<CHSELR1rs> {
        SQ2_W::new(self, 4)
    }
    /**Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W<CHSELR1rs> {
        SQ3_W::new(self, 8)
    }
    /**Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W<CHSELR1rs> {
        SQ4_W::new(self, 12)
    }
    /**Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<CHSELR1rs> {
        SQ5_W::new(self, 16)
    }
    /**Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<CHSELR1rs> {
        SQ6_W::new(self, 20)
    }
    /**Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<CHSELR1rs> {
        SQ7_W::new(self, 24)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<CHSELR1rs> {
        SQ8_W::new(self, 28)
    }
}
/**channel selection register CHSELRMOD = 1 in ADC_CFGR1

You can [`read`](crate::Reg::read) this register and get [`chselr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CHSELR1)*/
pub struct CHSELR1rs;
impl crate::RegisterSpec for CHSELR1rs {
    type Ux = u32;
}
///`read()` method returns [`chselr1::R`](R) reader structure
impl crate::Readable for CHSELR1rs {}
///`write(|w| ..)` method takes [`chselr1::W`](W) writer structure
impl crate::Writable for CHSELR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHSELR1 to value 0
impl crate::Resettable for CHSELR1rs {
    const RESET_VALUE: u32 = 0;
}
