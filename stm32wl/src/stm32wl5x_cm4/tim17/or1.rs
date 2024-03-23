#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1rs>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1rs>;
#[doc = "Timer 17 input 1 connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1_RMP {
    #[doc = "0: TI1 is connected to GPIO"]
    Gpio = 0,
    #[doc = "1: TI1 is connected to LSI"]
    Lsi = 1,
    #[doc = "2: TI1 is connected to LSE"]
    Lse = 2,
    #[doc = "3: TI1 is connected to RTC wake-up interrupt"]
    Rtc = 3,
}
impl From<TI1_RMP> for u8 {
    #[inline(always)]
    fn from(variant: TI1_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1_RMP {
    type Ux = u8;
}
#[doc = "Field `TI1_RMP` reader - Timer 17 input 1 connection"]
pub type TI1_RMP_R = crate::FieldReader<TI1_RMP>;
impl TI1_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1_RMP {
        match self.bits {
            0 => TI1_RMP::Gpio,
            1 => TI1_RMP::Lsi,
            2 => TI1_RMP::Lse,
            3 => TI1_RMP::Rtc,
            _ => unreachable!(),
        }
    }
    #[doc = "TI1 is connected to GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP::Gpio
    }
    #[doc = "TI1 is connected to LSI"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == TI1_RMP::Lsi
    }
    #[doc = "TI1 is connected to LSE"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == TI1_RMP::Lse
    }
    #[doc = "TI1 is connected to RTC wake-up interrupt"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == TI1_RMP::Rtc
    }
}
#[doc = "Field `TI1_RMP` writer - Timer 17 input 1 connection"]
pub type TI1_RMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TI1_RMP>;
impl<'a, REG> TI1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TI1 is connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Gpio)
    }
    #[doc = "TI1 is connected to LSI"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Lsi)
    }
    #[doc = "TI1 is connected to LSE"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Lse)
    }
    #[doc = "TI1 is connected to RTC wake-up interrupt"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Rtc)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer 17 input 1 connection"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer 17 input 1 connection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<OR1rs> {
        TI1_RMP_W::new(self, 0)
    }
}
#[doc = "TIM17 option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for OR1rs {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1rs {
    const RESET_VALUE: u32 = 0;
}
