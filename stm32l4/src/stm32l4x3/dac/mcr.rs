///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
/**DAC Channel 1 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1 {
    ///0: Normal mode - DAC channelx is connected to external pin with Buffer enabled
    NormalPinBuffer = 0,
    ///1: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    NormalPinChipBuffer = 1,
    ///2: Normal mode - DAC channelx is connected to external pin with Buffer disabled
    NormalPinNoBuffer = 2,
    ///3: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    NormalChipNoBuffer = 3,
    ///4: S&H mode - DAC channelx is connected to external pin with Buffer enabled
    ShpinBuffer = 4,
    ///5: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    ShpinChipBuffer = 5,
    ///6: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    ShpinNoBuffer = 6,
    ///7: S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
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
impl crate::IsEnum for MODE1 {}
///Field `MODE1` reader - DAC Channel 1 mode
pub type MODE1_R = crate::FieldReader<MODE1>;
impl MODE1_R {
    ///Get enumerated values variant
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
    ///Normal mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn is_normal_pin_buffer(&self) -> bool {
        *self == MODE1::NormalPinBuffer
    }
    ///Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn is_normal_pin_chip_buffer(&self) -> bool {
        *self == MODE1::NormalPinChipBuffer
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn is_normal_pin_no_buffer(&self) -> bool {
        *self == MODE1::NormalPinNoBuffer
    }
    ///Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn is_normal_chip_no_buffer(&self) -> bool {
        *self == MODE1::NormalChipNoBuffer
    }
    ///S&H mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn is_shpin_buffer(&self) -> bool {
        *self == MODE1::ShpinBuffer
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn is_shpin_chip_buffer(&self) -> bool {
        *self == MODE1::ShpinChipBuffer
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn is_shpin_no_buffer(&self) -> bool {
        *self == MODE1::ShpinNoBuffer
    }
    ///S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn is_shchip_no_buffer(&self) -> bool {
        *self == MODE1::ShchipNoBuffer
    }
}
///Field `MODE1` writer - DAC Channel 1 mode
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE1, crate::Safe>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinBuffer)
    }
    ///Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_chip_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinChipBuffer)
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn normal_pin_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinNoBuffer)
    }
    ///Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn normal_chip_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalChipNoBuffer)
    }
    ///S&H mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn shpin_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinBuffer)
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn shpin_chip_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinChipBuffer)
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shpin_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinNoBuffer)
    }
    ///S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shchip_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShchipNoBuffer)
    }
}
///Field `MODE2` reader - DAC Channel 2 mode
pub use MODE1_R as MODE2_R;
///Field `MODE2` writer - DAC Channel 2 mode
pub use MODE1_W as MODE2_W;
impl R {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - DAC Channel 2 mode
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<MCRrs> {
        MODE1_W::new(self, 0)
    }
    ///Bits 16:18 - DAC Channel 2 mode
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<MCRrs> {
        MODE2_W::new(self, 16)
    }
}
/**mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#DAC:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0;
}
