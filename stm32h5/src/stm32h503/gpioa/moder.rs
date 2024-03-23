#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODERrs>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODERrs>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0 {
    #[doc = "0: Input mode"]
    Input = 0,
    #[doc = "1: General purpose output mode"]
    Output = 1,
    #[doc = "2: Alternate function mode"]
    Alternate = 2,
    #[doc = "3: Analog mode"]
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
#[doc = "Field `MODE0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE0_R = crate::FieldReader<MODE0>;
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0::Input
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE0::Output
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODE0::Alternate
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODE0::Analog
    }
}
#[doc = "Field `MODE0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE0>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Input)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Output)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Alternate)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Analog)
    }
}
#[doc = "Field `MODE1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE1_R;
#[doc = "Field `MODE2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE2_R;
#[doc = "Field `MODE3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE3_R;
#[doc = "Field `MODE4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE4_R;
#[doc = "Field `MODE5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE5_R;
#[doc = "Field `MODE6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE6_R;
#[doc = "Field `MODE7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE7_R;
#[doc = "Field `MODE8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE8_R;
#[doc = "Field `MODE9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE9_R;
#[doc = "Field `MODE10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE10_R;
#[doc = "Field `MODE11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE11_R;
#[doc = "Field `MODE12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE12_R;
#[doc = "Field `MODE13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE13_R;
#[doc = "Field `MODE14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE14_R;
#[doc = "Field `MODE15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_R as MODE15_R;
#[doc = "Field `MODE1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE1_W;
#[doc = "Field `MODE2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE2_W;
#[doc = "Field `MODE3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE3_W;
#[doc = "Field `MODE4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE4_W;
#[doc = "Field `MODE5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE5_W;
#[doc = "Field `MODE6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE6_W;
#[doc = "Field `MODE7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE7_W;
#[doc = "Field `MODE8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE8_W;
#[doc = "Field `MODE9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE9_W;
#[doc = "Field `MODE10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE10_W;
#[doc = "Field `MODE11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE11_W;
#[doc = "Field `MODE12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE12_W;
#[doc = "Field `MODE13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE13_W;
#[doc = "Field `MODE14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE14_W;
#[doc = "Field `MODE15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use MODE0_W as MODE15_W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<MODERrs> {
        MODE0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<MODERrs> {
        MODE1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<MODERrs> {
        MODE2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<MODERrs> {
        MODE3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<MODERrs> {
        MODE4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<MODERrs> {
        MODE5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<MODERrs> {
        MODE6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<MODERrs> {
        MODE7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<MODERrs> {
        MODE8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<MODERrs> {
        MODE9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<MODERrs> {
        MODE10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<MODERrs> {
        MODE11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<MODERrs> {
        MODE12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<MODERrs> {
        MODE13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<MODERrs> {
        MODE14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<MODERrs> {
        MODE15_W::new(self, 30)
    }
}
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODERrs {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODER to value 0xabff_ffff"]
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xabff_ffff;
}
