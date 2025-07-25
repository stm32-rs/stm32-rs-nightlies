///Register `PRESC` reader
pub type R = crate::R<PRESCrs>;
///Register `PRESC` writer
pub type W = crate::W<PRESCrs>;
/**Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER {
    ///0: /1
    Div1 = 0,
    ///1: /2
    Div2 = 1,
    ///2: /4
    Div4 = 2,
    ///3: /6
    Div6 = 3,
    ///4: /8
    Div8 = 4,
    ///5: /10
    Div10 = 5,
    ///6: /12
    Div12 = 6,
    ///7: /16
    Div16 = 7,
    ///8: /32
    Div32 = 8,
    ///9: /64
    Div64 = 9,
    ///10: /128
    Div128 = 10,
    ///11: /256
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
impl crate::IsEnum for PRESCALER {}
///Field `PRESCALER` reader - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
pub type PRESCALER_R = crate::FieldReader<PRESCALER>;
impl PRESCALER_R {
    ///Get enumerated values variant
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
///Field `PRESCALER` writer - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
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
    ///Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESC")
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<PRESCrs> {
        PRESCALER_W::new(self, 0)
    }
}
/**LPUART prescaler register

You can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#LPUART1:PRESC)*/
pub struct PRESCrs;
impl crate::RegisterSpec for PRESCrs {
    type Ux = u32;
}
///`read()` method returns [`presc::R`](R) reader structure
impl crate::Readable for PRESCrs {}
///`write(|w| ..)` method takes [`presc::W`](W) writer structure
impl crate::Writable for PRESCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRESC to value 0
impl crate::Resettable for PRESCrs {}
