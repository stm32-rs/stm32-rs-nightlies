///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
/**DAC channel%s mode

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
///Field `MODE(1-2)` reader - DAC channel%s mode
pub type MODE_R = crate::FieldReader<MODE1>;
impl MODE_R {
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
///Field `MODE(1-2)` writer - DAC channel%s mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE1, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
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
impl R {
    ///DAC channel(1-2) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE1` field.</div>
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MODE_R::new(((self.bits >> (n * 16)) & 7) as u8)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) mode
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..2).map(move |n| MODE_R::new(((self.bits >> (n * 16)) & 7) as u8))
    }
    ///Bits 0:2 - DAC channel1 mode
    #[inline(always)]
    pub fn mode1(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - DAC channel2 mode
    #[inline(always)]
    pub fn mode2(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
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
    ///DAC channel(1-2) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE1` field.</div>
    #[inline(always)]
    pub fn mode(&mut self, n: u8) -> MODE_W<'_, MCRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MODE_W::new(self, n * 16)
    }
    ///Bits 0:2 - DAC channel1 mode
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE_W<'_, MCRrs> {
        MODE_W::new(self, 0)
    }
    ///Bits 16:18 - DAC channel2 mode
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE_W<'_, MCRrs> {
        MODE_W::new(self, 16)
    }
}
/**mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DAC:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCRrs {}
