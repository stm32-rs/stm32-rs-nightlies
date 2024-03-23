#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRHrs>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRHrs>;
#[doc = "Alternate function selection for port x bit y (y = 8..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL8 {
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
impl From<AFSEL8> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL8 {
    type Ux = u8;
}
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFSEL8_R = crate::FieldReader<AFSEL8>;
impl AFSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL8 {
        match self.bits {
            0 => AFSEL8::Af0,
            1 => AFSEL8::Af1,
            2 => AFSEL8::Af2,
            3 => AFSEL8::Af3,
            4 => AFSEL8::Af4,
            5 => AFSEL8::Af5,
            6 => AFSEL8::Af6,
            7 => AFSEL8::Af7,
            8 => AFSEL8::Af8,
            9 => AFSEL8::Af9,
            10 => AFSEL8::Af10,
            11 => AFSEL8::Af11,
            12 => AFSEL8::Af12,
            13 => AFSEL8::Af13,
            14 => AFSEL8::Af14,
            15 => AFSEL8::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFSEL8::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFSEL8::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFSEL8::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFSEL8::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFSEL8::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFSEL8::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFSEL8::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFSEL8::Af7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFSEL8::Af8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFSEL8::Af9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFSEL8::Af10
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFSEL8::Af11
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFSEL8::Af12
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFSEL8::Af13
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFSEL8::Af14
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFSEL8::Af15
    }
}
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFSEL8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, AFSEL8>;
impl<'a, REG> AFSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::Af15)
    }
}
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL9_R;
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL10_R;
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL11_R;
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL12_R;
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL13_R;
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL14_R;
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_R as AFSEL15_R;
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL9_W;
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL10_W;
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL11_W;
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL12_W;
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL13_W;
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL14_W;
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use AFSEL8_W as AFSEL15_W;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> AFSEL8_W<AFRHrs> {
        AFSEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> AFSEL9_W<AFRHrs> {
        AFSEL9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> AFSEL10_W<AFRHrs> {
        AFSEL10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> AFSEL11_W<AFRHrs> {
        AFSEL11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> AFSEL12_W<AFRHrs> {
        AFSEL12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> AFSEL13_W<AFRHrs> {
        AFSEL13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> AFSEL14_W<AFRHrs> {
        AFSEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> AFSEL15_W<AFRHrs> {
        AFSEL15_W::new(self, 28)
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
