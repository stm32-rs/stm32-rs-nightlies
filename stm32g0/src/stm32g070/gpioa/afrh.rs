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
#[doc = "Field `AFR(EL8,EL9,EL10,EL11,EL12,EL13,EL14,EL15)` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFR_R = crate::FieldReader<AFSEL8>;
impl AFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AFSEL8> {
        match self.bits {
            0 => Some(AFSEL8::Af0),
            1 => Some(AFSEL8::Af1),
            2 => Some(AFSEL8::Af2),
            3 => Some(AFSEL8::Af3),
            4 => Some(AFSEL8::Af4),
            5 => Some(AFSEL8::Af5),
            6 => Some(AFSEL8::Af6),
            7 => Some(AFSEL8::Af7),
            _ => None,
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
}
#[doc = "Field `AFR(EL8,EL9,EL10,EL11,EL12,EL13,EL14,EL15)` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL8>;
impl<'a, REG> AFR_W<'a, REG>
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
}
impl R {
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AFREL8` field"]
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel8(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel9(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel10(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel11(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel12(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel13(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel14(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrel15(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AFREL8` field"]
    #[inline(always)]
    #[must_use]
    pub fn afr(&mut self, n: u8) -> AFR_W<AFRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel8(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel9(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel10(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel11(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel12(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel13(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel14(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrel15(&mut self) -> AFR_W<AFRHrs> {
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
