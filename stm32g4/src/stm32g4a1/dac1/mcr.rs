#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCRrs>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCRrs>;
#[doc = "DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1 {
    #[doc = "0: Normal mode - DAC channelx is connected to external pin with Buffer enabled"]
    NormalPinBuffer = 0,
    #[doc = "1: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    NormalPinChipBuffer = 1,
    #[doc = "2: Normal mode - DAC channelx is connected to external pin with Buffer disabled"]
    NormalPinNoBuffer = 2,
    #[doc = "3: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    NormalChipNoBuffer = 3,
    #[doc = "4: S&amp;H mode - DAC channelx is connected to external pin with Buffer enabled"]
    ShpinBuffer = 4,
    #[doc = "5: S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    ShpinChipBuffer = 5,
    #[doc = "6: S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled"]
    ShpinNoBuffer = 6,
    #[doc = "7: S&amp;H mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    ShchipNoBuffer = 7,
}
impl From<MODE1> for u8 {
    #[inline(always)]
    fn from(variant: MODE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1 {
    type Ux = u8;
}
#[doc = "Field `MODE1` reader - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
pub type MODE1_R = crate::FieldReader<MODE1>;
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1 {
        match self.bits {
            0 => MODE1::NormalPinBuffer,
            1 => MODE1::NormalPinChipBuffer,
            2 => MODE1::NormalPinNoBuffer,
            3 => MODE1::NormalChipNoBuffer,
            4 => MODE1::ShpinBuffer,
            5 => MODE1::ShpinChipBuffer,
            6 => MODE1::ShpinNoBuffer,
            7 => MODE1::ShchipNoBuffer,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_normal_pin_buffer(&self) -> bool {
        *self == MODE1::NormalPinBuffer
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn is_normal_pin_chip_buffer(&self) -> bool {
        *self == MODE1::NormalPinChipBuffer
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn is_normal_pin_no_buffer(&self) -> bool {
        *self == MODE1::NormalPinNoBuffer
    }
    #[doc = "Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_normal_chip_no_buffer(&self) -> bool {
        *self == MODE1::NormalChipNoBuffer
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_shpin_buffer(&self) -> bool {
        *self == MODE1::ShpinBuffer
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn is_shpin_chip_buffer(&self) -> bool {
        *self == MODE1::ShpinChipBuffer
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_shpin_no_buffer(&self) -> bool {
        *self == MODE1::ShpinNoBuffer
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_shchip_no_buffer(&self) -> bool {
        *self == MODE1::ShchipNoBuffer
    }
}
#[doc = "Field `MODE1` writer - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
pub type MODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MODE1>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode - DAC channelx is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn normal_pin_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinBuffer)
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn normal_pin_chip_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinChipBuffer)
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn normal_pin_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinNoBuffer)
    }
    #[doc = "Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn normal_chip_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalChipNoBuffer)
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn shpin_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinBuffer)
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn shpin_chip_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinChipBuffer)
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn shpin_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinNoBuffer)
    }
    #[doc = "S&amp;H mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn shchip_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShchipNoBuffer)
    }
}
#[doc = "Field `DMADOUBLE1` reader - DAC Channel1 DMA double data mode"]
pub type DMADOUBLE1_R = crate::BitReader;
#[doc = "Field `DMADOUBLE1` writer - DAC Channel1 DMA double data mode"]
pub type DMADOUBLE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINFORMAT1` reader - Enable signed format for DAC channel1"]
pub type SINFORMAT1_R = crate::BitReader;
#[doc = "Field `SINFORMAT1` writer - Enable signed format for DAC channel1"]
pub type SINFORMAT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFSEL` reader - High frequency interface mode selection"]
pub type HFSEL_R = crate::FieldReader;
#[doc = "Field `HFSEL` writer - High frequency interface mode selection"]
pub type HFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE2` reader - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
pub use MODE1_R as MODE2_R;
#[doc = "Field `MODE2` writer - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
pub use MODE1_W as MODE2_W;
#[doc = "Field `DMADOUBLE2` reader - DAC Channel2 DMA double data mode"]
pub type DMADOUBLE2_R = crate::BitReader;
#[doc = "Field `DMADOUBLE2` writer - DAC Channel2 DMA double data mode"]
pub type DMADOUBLE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINFORMAT2` reader - Enable signed format for DAC channel2"]
pub type SINFORMAT2_R = crate::BitReader;
#[doc = "Field `SINFORMAT2` writer - Enable signed format for DAC channel2"]
pub type SINFORMAT2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<MCRrs> {
        MODE1_W::new(self, 0)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmadouble1(&mut self) -> DMADOUBLE1_W<MCRrs> {
        DMADOUBLE1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    #[must_use]
    pub fn sinformat1(&mut self) -> SINFORMAT1_W<MCRrs> {
        SINFORMAT1_W::new(self, 9)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn hfsel(&mut self) -> HFSEL_W<MCRrs> {
        HFSEL_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<MCRrs> {
        MODE2_W::new(self, 16)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmadouble2(&mut self) -> DMADOUBLE2_W<MCRrs> {
        DMADOUBLE2_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    #[must_use]
    pub fn sinformat2(&mut self) -> SINFORMAT2_W<MCRrs> {
        SINFORMAT2_W::new(self, 25)
    }
}
#[doc = "DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCRrs {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0;
}
