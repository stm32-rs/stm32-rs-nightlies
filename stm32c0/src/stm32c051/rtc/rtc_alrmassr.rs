///Register `RTC_ALRMASSR` reader
pub type R = crate::R<RTC_ALRMASSRrs>;
///Register `RTC_ALRMASSR` writer
pub type W = crate::W<RTC_ALRMASSRrs>;
///Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
pub type SS_R = crate::FieldReader<u16>;
///Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
/**Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKSS {
    ///0: No comparison on sub seconds for alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match).
    B0x0 = 0,
    ///1: SS\[14:1\] are don t care in alarm A comparison. Only SS\[0\] is compared.
    B0x1 = 1,
}
impl From<MASKSS> for u8 {
    #[inline(always)]
    fn from(variant: MASKSS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MASKSS {
    type Ux = u8;
}
impl crate::IsEnum for MASKSS {}
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
pub type MASKSS_R = crate::FieldReader<MASKSS>;
impl MASKSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MASKSS> {
        match self.bits {
            0 => Some(MASKSS::B0x0),
            1 => Some(MASKSS::B0x1),
            _ => None,
        }
    }
    ///No comparison on sub seconds for alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MASKSS::B0x0
    }
    ///SS\[14:1\] are don t care in alarm A comparison. Only SS\[0\] is compared.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MASKSS::B0x1
    }
}
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MASKSS>;
impl<'a, REG> MASKSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No comparison on sub seconds for alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS::B0x0)
    }
    ///SS\[14:1\] are don t care in alarm A comparison. Only SS\[0\] is compared.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS::B0x1)
    }
}
impl R {
    ///Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_ALRMASSR")
            .field("ss", &self.ss())
            .field("maskss", &self.maskss())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<'_, RTC_ALRMASSRrs> {
        SS_W::new(self, 0)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W<'_, RTC_ALRMASSRrs> {
        MASKSS_W::new(self, 24)
    }
}
/**RTC alarm A sub second register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrmassr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmassr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RTC:RTC_ALRMASSR)*/
pub struct RTC_ALRMASSRrs;
impl crate::RegisterSpec for RTC_ALRMASSRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_alrmassr::R`](R) reader structure
impl crate::Readable for RTC_ALRMASSRrs {}
///`write(|w| ..)` method takes [`rtc_alrmassr::W`](W) writer structure
impl crate::Writable for RTC_ALRMASSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC_ALRMASSR to value 0
impl crate::Resettable for RTC_ALRMASSRrs {}
