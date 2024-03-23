#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRHrs>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRHrs>;
#[doc = "Alternate function selection for port x bit y (y = 8..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFRH8 {
    #[doc = "0: AF0"]
    Af0 = 0,
    #[doc = "1: AF1"]
    Af1 = 1,
    #[doc = "2: AF2"]
    Af2 = 2,
    #[doc = "3: AF3"]
    Af3 = 3,
    #[doc = "4: AF4"]
    Af4 = 4,
    #[doc = "5: AF5"]
    Af5 = 5,
    #[doc = "6: AF6"]
    Af6 = 6,
    #[doc = "7: AF7"]
    Af7 = 7,
    #[doc = "8: AF8"]
    Af8 = 8,
    #[doc = "9: AF9"]
    Af9 = 9,
    #[doc = "10: AF10"]
    Af10 = 10,
    #[doc = "11: AF11"]
    Af11 = 11,
    #[doc = "12: AF12"]
    Af12 = 12,
    #[doc = "13: AF13"]
    Af13 = 13,
    #[doc = "14: AF14"]
    Af14 = 14,
    #[doc = "15: AF15"]
    Af15 = 15,
}
impl From<AFRH8> for u8 {
    #[inline(always)]
    fn from(variant: AFRH8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFRH8 {
    type Ux = u8;
}
#[doc = "Field `AFRH8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH8_R = crate::FieldReader<AFRH8>;
impl AFRH8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFRH8 {
        match self.bits {
            0 => AFRH8::Af0,
            1 => AFRH8::Af1,
            2 => AFRH8::Af2,
            3 => AFRH8::Af3,
            4 => AFRH8::Af4,
            5 => AFRH8::Af5,
            6 => AFRH8::Af6,
            7 => AFRH8::Af7,
            8 => AFRH8::Af8,
            9 => AFRH8::Af9,
            10 => AFRH8::Af10,
            11 => AFRH8::Af11,
            12 => AFRH8::Af12,
            13 => AFRH8::Af13,
            14 => AFRH8::Af14,
            15 => AFRH8::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRH8::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRH8::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRH8::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRH8::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRH8::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRH8::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRH8::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRH8::Af7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRH8::Af8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRH8::Af9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRH8::Af10
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRH8::Af11
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRH8::Af12
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRH8::Af13
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRH8::Af14
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRH8::Af15
    }
}
#[doc = "Field `AFRH8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, AFRH8>;
impl<'a, REG> AFRH8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af15)
    }
}
#[doc = "Field `AFRH9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH9_R;
#[doc = "Field `AFRH10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH10_R;
#[doc = "Field `AFRH11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH11_R;
#[doc = "Field `AFRH12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH12_R;
#[doc = "Field `AFRH13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH13_R;
#[doc = "Field `AFRH14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH14_R;
#[doc = "Field `AFRH15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_R as AFRH15_R;
#[doc = "Field `AFRH9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH9_W;
#[doc = "Field `AFRH10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH10_W;
#[doc = "Field `AFRH11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH11_W;
#[doc = "Field `AFRH12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH12_W;
#[doc = "Field `AFRH13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH13_W;
#[doc = "Field `AFRH14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH14_W;
#[doc = "Field `AFRH15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFRH8_W as AFRH15_W;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh8(&self) -> AFRH8_R {
        AFRH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh9(&self) -> AFRH9_R {
        AFRH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh10(&self) -> AFRH10_R {
        AFRH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh11(&self) -> AFRH11_R {
        AFRH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh12(&self) -> AFRH12_R {
        AFRH12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh13(&self) -> AFRH13_R {
        AFRH13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh14(&self) -> AFRH14_R {
        AFRH14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh15(&self) -> AFRH15_R {
        AFRH15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh8(&mut self) -> AFRH8_W<AFRHrs> {
        AFRH8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh9(&mut self) -> AFRH9_W<AFRHrs> {
        AFRH9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh10(&mut self) -> AFRH10_W<AFRHrs> {
        AFRH10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh11(&mut self) -> AFRH11_W<AFRHrs> {
        AFRH11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh12(&mut self) -> AFRH12_W<AFRHrs> {
        AFRH12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh13(&mut self) -> AFRH13_W<AFRHrs> {
        AFRH13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh14(&mut self) -> AFRH14_W<AFRHrs> {
        AFRH14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh15(&mut self) -> AFRH15_W<AFRHrs> {
        AFRH15_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AFRHrs {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRHrs {
    const RESET_VALUE: u32 = 0;
}
