#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2rs>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2rs>;
#[doc = "Channel 0 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0 {
    #[doc = "0: 3 cycles"]
    Cycles3 = 0,
    #[doc = "1: 15 cycles"]
    Cycles15 = 1,
    #[doc = "2: 28 cycles"]
    Cycles28 = 2,
    #[doc = "3: 56 cycles"]
    Cycles56 = 3,
    #[doc = "4: 84 cycles"]
    Cycles84 = 4,
    #[doc = "5: 112 cycles"]
    Cycles112 = 5,
    #[doc = "6: 144 cycles"]
    Cycles144 = 6,
    #[doc = "7: 480 cycles"]
    Cycles480 = 7,
}
impl From<SMP0> for u8 {
    #[inline(always)]
    fn from(variant: SMP0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP0 {
    type Ux = u8;
}
#[doc = "Field `SMP0` reader - Channel 0 sampling time selection"]
pub type SMP0_R = crate::FieldReader<SMP0>;
impl SMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP0 {
        match self.bits {
            0 => SMP0::Cycles3,
            1 => SMP0::Cycles15,
            2 => SMP0::Cycles28,
            3 => SMP0::Cycles56,
            4 => SMP0::Cycles84,
            5 => SMP0::Cycles112,
            6 => SMP0::Cycles144,
            7 => SMP0::Cycles480,
            _ => unreachable!(),
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMP0::Cycles3
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMP0::Cycles15
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMP0::Cycles28
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMP0::Cycles56
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMP0::Cycles84
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMP0::Cycles112
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMP0::Cycles144
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMP0::Cycles480
    }
}
#[doc = "Field `SMP0` writer - Channel 0 sampling time selection"]
pub type SMP0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP0>;
impl<'a, REG> SMP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles480)
    }
}
#[doc = "Field `SMP1` reader - Channel 1 sampling time selection"]
pub use SMP0_R as SMP1_R;
#[doc = "Field `SMP2` reader - Channel 2 sampling time selection"]
pub use SMP0_R as SMP2_R;
#[doc = "Field `SMP3` reader - Channel 3 sampling time selection"]
pub use SMP0_R as SMP3_R;
#[doc = "Field `SMP4` reader - Channel 4 sampling time selection"]
pub use SMP0_R as SMP4_R;
#[doc = "Field `SMP5` reader - Channel 5 sampling time selection"]
pub use SMP0_R as SMP5_R;
#[doc = "Field `SMP6` reader - Channel 6 sampling time selection"]
pub use SMP0_R as SMP6_R;
#[doc = "Field `SMP7` reader - Channel 7 sampling time selection"]
pub use SMP0_R as SMP7_R;
#[doc = "Field `SMP8` reader - Channel 8 sampling time selection"]
pub use SMP0_R as SMP8_R;
#[doc = "Field `SMP9` reader - Channel 9 sampling time selection"]
pub use SMP0_R as SMP9_R;
#[doc = "Field `SMP1` writer - Channel 1 sampling time selection"]
pub use SMP0_W as SMP1_W;
#[doc = "Field `SMP2` writer - Channel 2 sampling time selection"]
pub use SMP0_W as SMP2_W;
#[doc = "Field `SMP3` writer - Channel 3 sampling time selection"]
pub use SMP0_W as SMP3_W;
#[doc = "Field `SMP4` writer - Channel 4 sampling time selection"]
pub use SMP0_W as SMP4_W;
#[doc = "Field `SMP5` writer - Channel 5 sampling time selection"]
pub use SMP0_W as SMP5_W;
#[doc = "Field `SMP6` writer - Channel 6 sampling time selection"]
pub use SMP0_W as SMP6_W;
#[doc = "Field `SMP7` writer - Channel 7 sampling time selection"]
pub use SMP0_W as SMP7_W;
#[doc = "Field `SMP8` writer - Channel 8 sampling time selection"]
pub use SMP0_W as SMP8_W;
#[doc = "Field `SMP9` writer - Channel 9 sampling time selection"]
pub use SMP0_W as SMP9_W;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<SMPR2rs> {
        SMP0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPR2rs> {
        SMP1_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPR2rs> {
        SMP2_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<SMPR2rs> {
        SMP3_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<SMPR2rs> {
        SMP4_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<SMPR2rs> {
        SMP5_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<SMPR2rs> {
        SMP6_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<SMPR2rs> {
        SMP7_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<SMPR2rs> {
        SMP8_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<SMPR2rs> {
        SMP9_W::new(self, 27)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2rs {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2rs {
    const RESET_VALUE: u32 = 0;
}
