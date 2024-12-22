///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
/**Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0 {
    ///0: Input mode
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODE0> for u8 {
    #[inline(always)]
    fn from(variant: MODE0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0 {
    type Ux = u8;
}
impl crate::IsEnum for MODE0 {}
///Field `MODE0` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type MODE0_R = crate::FieldReader<MODE0>;
impl MODE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE0 {
        match self.bits {
            0 => MODE0::Input,
            1 => MODE0::Output,
            2 => MODE0::Alternate,
            3 => MODE0::Analog,
            _ => unreachable!(),
        }
    }
    ///Input mode
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0::Input
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE0::Output
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODE0::Alternate
    }
    ///Analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODE0::Analog
    }
}
///Field `MODE0` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type MODE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE0, crate::Safe>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Analog)
    }
}
///Field `MODE1` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE1_R;
///Field `MODE2` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE2_R;
///Field `MODE3` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE3_R;
///Field `MODE4` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE4_R;
///Field `MODE5` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE5_R;
///Field `MODE6` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE6_R;
///Field `MODE7` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE7_R;
///Field `MODE8` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE8_R;
///Field `MODE9` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE9_R;
///Field `MODE10` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE10_R;
///Field `MODE11` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE11_R;
///Field `MODE12` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE12_R;
///Field `MODE13` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE13_R;
///Field `MODE14` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE14_R;
///Field `MODE15` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_R as MODE15_R;
///Field `MODE1` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE1_W;
///Field `MODE2` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE2_W;
///Field `MODE3` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE3_W;
///Field `MODE4` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE4_W;
///Field `MODE5` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE5_W;
///Field `MODE6` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE6_W;
///Field `MODE7` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE7_W;
///Field `MODE8` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE8_W;
///Field `MODE9` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE9_W;
///Field `MODE10` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE10_W;
///Field `MODE11` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE11_W;
///Field `MODE12` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE12_W;
///Field `MODE13` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE13_W;
///Field `MODE14` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE14_W;
///Field `MODE15` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use MODE0_W as MODE15_W;
impl R {
    ///Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("mode0", &self.mode0())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("mode4", &self.mode4())
            .field("mode5", &self.mode5())
            .field("mode6", &self.mode6())
            .field("mode7", &self.mode7())
            .field("mode8", &self.mode8())
            .field("mode9", &self.mode9())
            .field("mode10", &self.mode10())
            .field("mode11", &self.mode11())
            .field("mode12", &self.mode12())
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W<MODERrs> {
        MODE0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<MODERrs> {
        MODE1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<MODERrs> {
        MODE2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W<MODERrs> {
        MODE3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W<MODERrs> {
        MODE4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W<MODERrs> {
        MODE5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W<MODERrs> {
        MODE6_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W<MODERrs> {
        MODE7_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W<MODERrs> {
        MODE8_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W<MODERrs> {
        MODE9_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W<MODERrs> {
        MODE10_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W<MODERrs> {
        MODE11_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W<MODERrs> {
        MODE12_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W<MODERrs> {
        MODE13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W<MODERrs> {
        MODE14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W<MODERrs> {
        MODE15_W::new(self, 30)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GPIOB:MODER)*/
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
///`reset()` method sets MODER to value 0xffff_febf
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xffff_febf;
}
