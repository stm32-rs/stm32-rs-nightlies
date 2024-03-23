#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCRrs>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCRrs>;
#[doc = "DAC Channel 1 mode\n\nValue on reset: 0"]
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
#[doc = "Field `MODE1` reader - DAC Channel 1 mode"]
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
#[doc = "Field `MODE1` writer - DAC Channel 1 mode"]
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
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<MCRrs> {
        MODE1_W::new(self, 0)
    }
}
#[doc = "mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
