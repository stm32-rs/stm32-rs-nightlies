///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
/**Port configuration I/O pin 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTPUT_SPEED {
    ///0: Low speed
    LowSpeed = 0,
    ///1: Medium speed
    MediumSpeed = 1,
    ///2: High speed
    HighSpeed = 2,
    ///3: Very high speed
    VeryHighSpeed = 3,
}
impl From<OUTPUT_SPEED> for u8 {
    #[inline(always)]
    fn from(variant: OUTPUT_SPEED) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUTPUT_SPEED {
    type Ux = u8;
}
impl crate::IsEnum for OUTPUT_SPEED {}
///Field `OSPEED0` reader - Port configuration I/O pin 0
pub type OSPEED0_R = crate::FieldReader<OUTPUT_SPEED>;
impl OSPEED0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTPUT_SPEED {
        match self.bits {
            0 => OUTPUT_SPEED::LowSpeed,
            1 => OUTPUT_SPEED::MediumSpeed,
            2 => OUTPUT_SPEED::HighSpeed,
            3 => OUTPUT_SPEED::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    ///Low speed
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OUTPUT_SPEED::LowSpeed
    }
    ///Medium speed
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OUTPUT_SPEED::MediumSpeed
    }
    ///High speed
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OUTPUT_SPEED::HighSpeed
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OUTPUT_SPEED::VeryHighSpeed
    }
}
///Field `OSPEED0` writer - Port configuration I/O pin 0
pub type OSPEED0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OUTPUT_SPEED, crate::Safe>;
impl<'a, REG> OSPEED0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_SPEED::LowSpeed)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_SPEED::MediumSpeed)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_SPEED::HighSpeed)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_SPEED::VeryHighSpeed)
    }
}
///Field `OSPEED1` reader - Port configuration I/O pin 1
pub use OSPEED0_R as OSPEED1_R;
///Field `OSPEED2` reader - Port configuration I/O pin 2
pub use OSPEED0_R as OSPEED2_R;
///Field `OSPEED3` reader - Port configuration I/O pin 3
pub use OSPEED0_R as OSPEED3_R;
///Field `OSPEED5` reader - Port configuration I/O pin 5
pub use OSPEED0_R as OSPEED5_R;
///Field `OSPEED6` reader - Port configuration I/O pin 6
pub use OSPEED0_R as OSPEED6_R;
///Field `OSPEED7` reader - Port configuration I/O pin 7
pub use OSPEED0_R as OSPEED7_R;
///Field `OSPEED8` reader - Port configuration I/O pin 8
pub use OSPEED0_R as OSPEED8_R;
///Field `OSPEED9` reader - Port configuration I/O pin 9
pub use OSPEED0_R as OSPEED9_R;
///Field `OSPEED10` reader - Port configuration I/O pin 10
pub use OSPEED0_R as OSPEED10_R;
///Field `OSPEED11` reader - Port configuration I/O pin 11
pub use OSPEED0_R as OSPEED11_R;
///Field `OSPEED12` reader - Port configuration I/O pin 12
pub use OSPEED0_R as OSPEED12_R;
///Field `OSPEED13` reader - Port configuration I/O pin 13
pub use OSPEED0_R as OSPEED13_R;
///Field `OSPEED14` reader - Port configuration I/O pin 14
pub use OSPEED0_R as OSPEED14_R;
///Field `OSPEED15` reader - Port configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOA SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub use OSPEED0_R as OSPEED15_R;
///Field `OSPEED1` writer - Port configuration I/O pin 1
pub use OSPEED0_W as OSPEED1_W;
///Field `OSPEED2` writer - Port configuration I/O pin 2
pub use OSPEED0_W as OSPEED2_W;
///Field `OSPEED3` writer - Port configuration I/O pin 3
pub use OSPEED0_W as OSPEED3_W;
///Field `OSPEED5` writer - Port configuration I/O pin 5
pub use OSPEED0_W as OSPEED5_W;
///Field `OSPEED6` writer - Port configuration I/O pin 6
pub use OSPEED0_W as OSPEED6_W;
///Field `OSPEED7` writer - Port configuration I/O pin 7
pub use OSPEED0_W as OSPEED7_W;
///Field `OSPEED8` writer - Port configuration I/O pin 8
pub use OSPEED0_W as OSPEED8_W;
///Field `OSPEED9` writer - Port configuration I/O pin 9
pub use OSPEED0_W as OSPEED9_W;
///Field `OSPEED10` writer - Port configuration I/O pin 10
pub use OSPEED0_W as OSPEED10_W;
///Field `OSPEED11` writer - Port configuration I/O pin 11
pub use OSPEED0_W as OSPEED11_W;
///Field `OSPEED12` writer - Port configuration I/O pin 12
pub use OSPEED0_W as OSPEED12_W;
///Field `OSPEED13` writer - Port configuration I/O pin 13
pub use OSPEED0_W as OSPEED13_W;
///Field `OSPEED14` writer - Port configuration I/O pin 14
pub use OSPEED0_W as OSPEED14_W;
///Field `OSPEED15` writer - Port configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOA SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub use OSPEED0_W as OSPEED15_W;
impl R {
    ///Bits 0:1 - Port configuration I/O pin 0
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port configuration I/O pin 1
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port configuration I/O pin 2
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port configuration I/O pin 3
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 10:11 - Port configuration I/O pin 5
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port configuration I/O pin 6
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port configuration I/O pin 7
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port configuration I/O pin 8
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port configuration I/O pin 9
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port configuration I/O pin 10
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port configuration I/O pin 11
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port configuration I/O pin 12
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port configuration I/O pin 13
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port configuration I/O pin 14
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOA SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed0", &self.ospeed0())
            .field("ospeed1", &self.ospeed1())
            .field("ospeed2", &self.ospeed2())
            .field("ospeed3", &self.ospeed3())
            .field("ospeed5", &self.ospeed5())
            .field("ospeed6", &self.ospeed6())
            .field("ospeed7", &self.ospeed7())
            .field("ospeed8", &self.ospeed8())
            .field("ospeed9", &self.ospeed9())
            .field("ospeed10", &self.ospeed10())
            .field("ospeed11", &self.ospeed11())
            .field("ospeed12", &self.ospeed12())
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port configuration I/O pin 0
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W<'_, OSPEEDRrs> {
        OSPEED0_W::new(self, 0)
    }
    ///Bits 2:3 - Port configuration I/O pin 1
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W<'_, OSPEEDRrs> {
        OSPEED1_W::new(self, 2)
    }
    ///Bits 4:5 - Port configuration I/O pin 2
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W<'_, OSPEEDRrs> {
        OSPEED2_W::new(self, 4)
    }
    ///Bits 6:7 - Port configuration I/O pin 3
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W<'_, OSPEEDRrs> {
        OSPEED3_W::new(self, 6)
    }
    ///Bits 10:11 - Port configuration I/O pin 5
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W<'_, OSPEEDRrs> {
        OSPEED5_W::new(self, 10)
    }
    ///Bits 12:13 - Port configuration I/O pin 6
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W<'_, OSPEEDRrs> {
        OSPEED6_W::new(self, 12)
    }
    ///Bits 14:15 - Port configuration I/O pin 7
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W<'_, OSPEEDRrs> {
        OSPEED7_W::new(self, 14)
    }
    ///Bits 16:17 - Port configuration I/O pin 8
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W<'_, OSPEEDRrs> {
        OSPEED8_W::new(self, 16)
    }
    ///Bits 18:19 - Port configuration I/O pin 9
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED9_W<'_, OSPEEDRrs> {
        OSPEED9_W::new(self, 18)
    }
    ///Bits 20:21 - Port configuration I/O pin 10
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED10_W<'_, OSPEEDRrs> {
        OSPEED10_W::new(self, 20)
    }
    ///Bits 22:23 - Port configuration I/O pin 11
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED11_W<'_, OSPEEDRrs> {
        OSPEED11_W::new(self, 22)
    }
    ///Bits 24:25 - Port configuration I/O pin 12
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED12_W<'_, OSPEEDRrs> {
        OSPEED12_W::new(self, 24)
    }
    ///Bits 26:27 - Port configuration I/O pin 13
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W<'_, OSPEEDRrs> {
        OSPEED13_W::new(self, 26)
    }
    ///Bits 28:29 - Port configuration I/O pin 14
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W<'_, OSPEEDRrs> {
        OSPEED14_W::new(self, 28)
    }
    ///Bits 30:31 - Port configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOA SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W<'_, OSPEEDRrs> {
        OSPEED15_W::new(self, 30)
    }
}
/**GPIO port A output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOA:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSPEEDR to value 0x0800_0000
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
