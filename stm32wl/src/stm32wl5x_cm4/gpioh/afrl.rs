#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRLrs>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRLrs>;
#[doc = "Alternate function selection for port x bit y (y = 0..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFRL3 {
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
impl From<AFRL3> for u8 {
    #[inline(always)]
    fn from(variant: AFRL3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFRL3 {
    type Ux = u8;
}
#[doc = "Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_R = crate::FieldReader<AFRL3>;
impl AFRL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFRL3 {
        match self.bits {
            0 => AFRL3::Af0,
            1 => AFRL3::Af1,
            2 => AFRL3::Af2,
            3 => AFRL3::Af3,
            4 => AFRL3::Af4,
            5 => AFRL3::Af5,
            6 => AFRL3::Af6,
            7 => AFRL3::Af7,
            8 => AFRL3::Af8,
            9 => AFRL3::Af9,
            10 => AFRL3::Af10,
            11 => AFRL3::Af11,
            12 => AFRL3::Af12,
            13 => AFRL3::Af13,
            14 => AFRL3::Af14,
            15 => AFRL3::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRL3::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRL3::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRL3::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRL3::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRL3::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRL3::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRL3::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRL3::Af7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRL3::Af8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRL3::Af9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRL3::Af10
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRL3::Af11
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRL3::Af12
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRL3::Af13
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRL3::Af14
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRL3::Af15
    }
}
#[doc = "Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, AFRL3>;
impl<'a, REG> AFRL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(AFRL3::Af15)
    }
}
impl R {
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl3(&mut self) -> AFRL3_W<AFRLrs> {
        AFRL3_W::new(self, 12)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AFRLrs {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRLrs {
    const RESET_VALUE: u32 = 0;
}
