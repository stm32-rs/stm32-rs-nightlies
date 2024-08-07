///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
/**Port x configuration bits (y = 0..15)

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODER0 {
    ///0: Input mode (reset state)
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODER0> for u8 {
    #[inline(always)]
    fn from(variant: MODER0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODER0 {
    type Ux = u8;
}
impl crate::IsEnum for MODER0 {}
///Field `MODER0` reader - Port x configuration bits (y = 0..15)
pub type MODER0_R = crate::FieldReader<MODER0>;
impl MODER0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODER0 {
        match self.bits {
            0 => MODER0::Input,
            1 => MODER0::Output,
            2 => MODER0::Alternate,
            3 => MODER0::Analog,
            _ => unreachable!(),
        }
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODER0::Input
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODER0::Output
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODER0::Alternate
    }
    ///Analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODER0::Analog
    }
}
///Field `MODER0` writer - Port x configuration bits (y = 0..15)
pub type MODER0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODER0, crate::Safe>;
impl<'a, REG> MODER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Analog)
    }
}
///Field `MODER1` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER1_R;
///Field `MODER2` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER2_R;
///Field `MODER3` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER3_R;
///Field `MODER4` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER4_R;
///Field `MODER5` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER5_R;
///Field `MODER6` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER6_R;
///Field `MODER13` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER13_R;
///Field `MODER14` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER14_R;
///Field `MODER15` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER15_R;
///Field `MODER1` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER1_W;
///Field `MODER2` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER2_W;
///Field `MODER3` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER3_W;
///Field `MODER4` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER4_W;
///Field `MODER5` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER5_W;
///Field `MODER6` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER6_W;
///Field `MODER13` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER13_W;
///Field `MODER14` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER14_W;
///Field `MODER15` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER15_W;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("moder0", &self.moder0())
            .field("moder15", &self.moder15())
            .field("moder14", &self.moder14())
            .field("moder13", &self.moder13())
            .field("moder6", &self.moder6())
            .field("moder5", &self.moder5())
            .field("moder4", &self.moder4())
            .field("moder3", &self.moder3())
            .field("moder2", &self.moder2())
            .field("moder1", &self.moder1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<MODERrs> {
        MODER0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<MODERrs> {
        MODER1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<MODERrs> {
        MODER2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<MODERrs> {
        MODER3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<MODERrs> {
        MODER4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<MODERrs> {
        MODER5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<MODERrs> {
        MODER6_W::new(self, 12)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER13_W<MODERrs> {
        MODER13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER14_W<MODERrs> {
        MODER14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder15(&mut self) -> MODER15_W<MODERrs> {
        MODER15_W::new(self, 30)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#GPIOC:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODER to value 0xfc00_3fff
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xfc00_3fff;
}
