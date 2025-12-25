///Register `PR` reader
pub type R = crate::R<PRrs>;
///Register `PR` writer
pub type W = crate::W<PRrs>;
/**Prescaler divider

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR {
    ///0: Divider /4
    DivideBy4 = 0,
    ///1: Divider /8
    DivideBy8 = 1,
    ///2: Divider /16
    DivideBy16 = 2,
    ///3: Divider /32
    DivideBy32 = 3,
    ///4: Divider /64
    DivideBy64 = 4,
    ///5: Divider /128
    DivideBy128 = 5,
    ///6: Divider /256
    DivideBy256 = 6,
}
impl From<PR> for u8 {
    #[inline(always)]
    fn from(variant: PR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR {
    type Ux = u8;
}
impl crate::IsEnum for PR {}
///Field `PR` reader - Prescaler divider
pub type PR_R = crate::FieldReader<PR>;
impl PR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR {
        match self.bits {
            0 => PR::DivideBy4,
            1 => PR::DivideBy8,
            2 => PR::DivideBy16,
            3 => PR::DivideBy32,
            4 => PR::DivideBy64,
            5 => PR::DivideBy128,
            _ => PR::DivideBy256,
        }
    }
    ///Divider /4
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PR::DivideBy4
    }
    ///Divider /8
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PR::DivideBy8
    }
    ///Divider /16
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PR::DivideBy16
    }
    ///Divider /32
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PR::DivideBy32
    }
    ///Divider /64
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PR::DivideBy64
    }
    ///Divider /128
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PR::DivideBy128
    }
    ///Divider /256
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        matches!(self.variant(), PR::DivideBy256)
    }
}
///Field `PR` writer - Prescaler divider
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PR, crate::Safe>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divider /4
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy4)
    }
    ///Divider /8
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy8)
    }
    ///Divider /16
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy16)
    }
    ///Divider /32
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy32)
    }
    ///Divider /64
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy64)
    }
    ///Divider /128
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy128)
    }
    ///Divider /256
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy256)
    }
}
impl R {
    ///Bits 0:2 - Prescaler divider
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR").field("pr", &self.pr()).finish()
    }
}
impl W {
    ///Bits 0:2 - Prescaler divider
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 0)
    }
}
/**Prescaler register

You can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#IWDG:PR)*/
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u16;
}
///`read()` method returns [`pr::R`](R) reader structure
impl crate::Readable for PRrs {}
///`write(|w| ..)` method takes [`pr::W`](W) writer structure
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PR to value 0
impl crate::Resettable for PRrs {}
