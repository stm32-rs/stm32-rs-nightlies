///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
/**Port x configuration bits (y = 0..15)

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Input mode (reset state)
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODER0` reader - Port x configuration bits (y = 0..15)
pub type MODER0_R = crate::FieldReader<MODE>;
impl MODER0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Input,
            1 => MODE::Output,
            2 => MODE::Alternate,
            3 => MODE::Analog,
            _ => unreachable!(),
        }
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE::Input
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE::Output
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODE::Alternate
    }
    ///Analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODE::Analog
    }
}
///Field `MODER0` writer - Port x configuration bits (y = 0..15)
pub type MODER0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Analog)
    }
}
///Field `MODER1` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER1_R;
///Field `MODER3` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER3_R;
///Field `MODER1` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER1_W;
///Field `MODER3` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER3_W;
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
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("moder0", &self.moder0())
            .field("moder3", &self.moder3())
            .field("moder1", &self.moder1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder0(&mut self) -> MODER0_W<'_, MODERrs> {
        MODER0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder1(&mut self) -> MODER1_W<'_, MODERrs> {
        MODER1_W::new(self, 2)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER3_W<'_, MODERrs> {
        MODER3_W::new(self, 6)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MODER to value 0xcf
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xcf;
}
