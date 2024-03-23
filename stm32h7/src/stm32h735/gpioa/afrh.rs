#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRHrs>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRHrs>;
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFR8 {
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
impl From<AFR8> for u8 {
    #[inline(always)]
    fn from(variant: AFR8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFR8 {
    type Ux = u8;
}
#[doc = "Field `AFR(8-15)` reader - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
pub type AFR_R = crate::FieldReader<AFR8>;
impl AFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFR8 {
        match self.bits {
            0 => AFR8::Af0,
            1 => AFR8::Af1,
            2 => AFR8::Af2,
            3 => AFR8::Af3,
            4 => AFR8::Af4,
            5 => AFR8::Af5,
            6 => AFR8::Af6,
            7 => AFR8::Af7,
            8 => AFR8::Af8,
            9 => AFR8::Af9,
            10 => AFR8::Af10,
            11 => AFR8::Af11,
            12 => AFR8::Af12,
            13 => AFR8::Af13,
            14 => AFR8::Af14,
            15 => AFR8::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFR8::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFR8::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFR8::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFR8::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFR8::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFR8::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFR8::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFR8::Af7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFR8::Af8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFR8::Af9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFR8::Af10
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFR8::Af11
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFR8::Af12
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFR8::Af13
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFR8::Af14
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFR8::Af15
    }
}
#[doc = "Field `AFR(8-15)` writer - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
pub type AFR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, AFR8>;
impl<'a, REG> AFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(AFR8::Af15)
    }
}
impl R {
    #[doc = "3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AFR8` field"]
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AFR8` field"]
    #[inline(always)]
    #[must_use]
    pub fn afr(&mut self, n: u8) -> AFR_W<AFRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr8(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr9(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr10(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr11(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr12(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr13(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr14(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afr15(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 28)
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
