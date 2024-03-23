#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRLrs>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRLrs>;
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFR0 {
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
impl From<AFR0> for u8 {
    #[inline(always)]
    fn from(variant: AFR0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFR0 {
    type Ux = u8;
}
#[doc = "Field `AFR(0-7)` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR_R = crate::FieldReader<AFR0>;
impl AFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFR0 {
        match self.bits {
            0 => AFR0::Af0,
            1 => AFR0::Af1,
            2 => AFR0::Af2,
            3 => AFR0::Af3,
            4 => AFR0::Af4,
            5 => AFR0::Af5,
            6 => AFR0::Af6,
            7 => AFR0::Af7,
            8 => AFR0::Af8,
            9 => AFR0::Af9,
            10 => AFR0::Af10,
            11 => AFR0::Af11,
            12 => AFR0::Af12,
            13 => AFR0::Af13,
            14 => AFR0::Af14,
            15 => AFR0::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFR0::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFR0::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFR0::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFR0::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFR0::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFR0::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFR0::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFR0::Af7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFR0::Af8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFR0::Af9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFR0::Af10
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFR0::Af11
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFR0::Af12
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFR0::Af13
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFR0::Af14
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFR0::Af15
    }
}
#[doc = "Field `AFR(0-7)` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, AFR0>;
impl<'a, REG> AFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(AFR0::Af15)
    }
}
impl R {
    #[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AFR0` field"]
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AFR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn afr(&mut self, n: u8) -> AFR_W<AFRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr0(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr1(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr2(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr3(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr4(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr5(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr6(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr7(&mut self) -> AFR_W<AFRLrs> {
        AFR_W::new(self, 28)
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
