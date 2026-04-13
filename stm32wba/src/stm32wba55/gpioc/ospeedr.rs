///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
/**Port C configuration I/O pin 13

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
///Field `OSPEED13` reader - Port C configuration I/O pin 13
pub type OSPEED13_R = crate::FieldReader<OUTPUT_SPEED>;
impl OSPEED13_R {
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
///Field `OSPEED13` writer - Port C configuration I/O pin 13
pub type OSPEED13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OUTPUT_SPEED, crate::Safe>;
impl<'a, REG> OSPEED13_W<'a, REG>
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
///Field `OSPEED14` reader - Port C configuration I/O pin 14
pub use OSPEED13_R as OSPEED14_R;
///Field `OSPEED15` reader - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub use OSPEED13_R as OSPEED15_R;
///Field `OSPEED14` writer - Port C configuration I/O pin 14
pub use OSPEED13_W as OSPEED14_W;
///Field `OSPEED15` writer - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub use OSPEED13_W as OSPEED15_W;
impl R {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W<'_, OSPEEDRrs> {
        OSPEED13_W::new(self, 26)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W<'_, OSPEEDRrs> {
        OSPEED14_W::new(self, 28)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W<'_, OSPEEDRrs> {
        OSPEED15_W::new(self, 30)
    }
}
/**GPIOC port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOC:OSPEEDR)*/
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
///`reset()` method sets OSPEEDR to value 0
impl crate::Resettable for OSPEEDRrs {}
