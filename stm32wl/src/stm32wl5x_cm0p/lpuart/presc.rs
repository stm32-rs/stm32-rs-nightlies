#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PRESCrs>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PRESCrs>;
#[doc = "Clock prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER {
    #[doc = "0: /1"]
    Div1 = 0,
    #[doc = "1: /2"]
    Div2 = 1,
    #[doc = "2: /4"]
    Div4 = 2,
    #[doc = "3: /6"]
    Div6 = 3,
    #[doc = "4: /8"]
    Div8 = 4,
    #[doc = "5: /10"]
    Div10 = 5,
    #[doc = "6: /12"]
    Div12 = 6,
    #[doc = "7: /16"]
    Div16 = 7,
    #[doc = "8: /32"]
    Div32 = 8,
    #[doc = "9: /64"]
    Div64 = 9,
    #[doc = "10: /128"]
    Div128 = 10,
    #[doc = "11: /256"]
    Div256 = 11,
}
impl From<PRESCALER> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER {
    type Ux = u8;
}
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub type PRESCALER_R = crate::FieldReader<PRESCALER>;
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCALER> {
        match self.bits {
            0 => Some(PRESCALER::Div1),
            1 => Some(PRESCALER::Div2),
            2 => Some(PRESCALER::Div4),
            3 => Some(PRESCALER::Div6),
            4 => Some(PRESCALER::Div8),
            5 => Some(PRESCALER::Div10),
            6 => Some(PRESCALER::Div12),
            7 => Some(PRESCALER::Div16),
            8 => Some(PRESCALER::Div32),
            9 => Some(PRESCALER::Div64),
            10 => Some(PRESCALER::Div128),
            11 => Some(PRESCALER::Div256),
            _ => None,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER::Div1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER::Div2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER::Div4
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESCALER::Div6
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER::Div8
    }
    #[doc = "/10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESCALER::Div10
    }
    #[doc = "/12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESCALER::Div12
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER::Div16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER::Div32
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER::Div64
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER::Div128
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER::Div256
    }
}
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESCALER>;
impl<'a, REG> PRESCALER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div8)
    }
    #[doc = "/10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div10)
    }
    #[doc = "/12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div12)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div128)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::Div256)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<PRESCrs> {
        PRESCALER_W::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESCrs;
impl crate::RegisterSpec for PRESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PRESCrs {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESCrs {
    const RESET_VALUE: u32 = 0;
}
