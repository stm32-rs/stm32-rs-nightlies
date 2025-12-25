///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
/**Port x configuration bits (y = 0..15)

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
///Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_R = crate::FieldReader<OUTPUT_SPEED>;
impl OSPEEDR0_R {
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
///Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OUTPUT_SPEED, crate::Safe>;
impl<'a, REG> OSPEEDR0_W<'a, REG>
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
///Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR1_R;
///Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR2_R;
///Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR3_R;
///Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR4_R;
///Field `OSPEEDR5` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR5_R;
///Field `OSPEEDR6` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR6_R;
///Field `OSPEEDR13` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR13_R;
///Field `OSPEEDR14` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR14_R;
///Field `OSPEEDR15` reader - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_R as OSPEEDR15_R;
///Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR1_W;
///Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR2_W;
///Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR3_W;
///Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR4_W;
///Field `OSPEEDR5` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR5_W;
///Field `OSPEEDR6` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR6_W;
///Field `OSPEEDR13` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR13_W;
///Field `OSPEEDR14` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR14_W;
///Field `OSPEEDR15` writer - Port x configuration bits (y = 0..15)
pub use OSPEEDR0_W as OSPEEDR15_W;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeedr0", &self.ospeedr0())
            .field("ospeedr15", &self.ospeedr15())
            .field("ospeedr14", &self.ospeedr14())
            .field("ospeedr13", &self.ospeedr13())
            .field("ospeedr6", &self.ospeedr6())
            .field("ospeedr5", &self.ospeedr5())
            .field("ospeedr4", &self.ospeedr4())
            .field("ospeedr3", &self.ospeedr3())
            .field("ospeedr2", &self.ospeedr2())
            .field("ospeedr1", &self.ospeedr1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<'_, OSPEEDRrs> {
        OSPEEDR0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<'_, OSPEEDRrs> {
        OSPEEDR1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<'_, OSPEEDRrs> {
        OSPEEDR2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<'_, OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<'_, OSPEEDRrs> {
        OSPEEDR4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W<'_, OSPEEDRrs> {
        OSPEEDR5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W<'_, OSPEEDRrs> {
        OSPEEDR6_W::new(self, 12)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W<'_, OSPEEDRrs> {
        OSPEEDR13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W<'_, OSPEEDRrs> {
        OSPEEDR14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W<'_, OSPEEDRrs> {
        OSPEEDR15_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#GPIOC:OSPEEDR)*/
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
