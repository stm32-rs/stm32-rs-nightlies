///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
/**Timer 17 input 1 connection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1_RMP {
    ///0: TI1 is connected to GPIO
    Gpio = 0,
    ///1: TI1 is connected to LSI
    Lsi = 1,
    ///2: TI1 is connected to LSE
    Lse = 2,
    ///3: TI1 is connected to RTC wake-up interrupt
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
impl crate::IsEnum for TI1_RMP {}
///Field `TI1_RMP` reader - Timer 17 input 1 connection
pub type TI1_RMP_R = crate::FieldReader<TI1_RMP>;
impl TI1_RMP_R {
    ///Get enumerated values variant
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
    ///TI1 is connected to GPIO
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP::Gpio
    }
    ///TI1 is connected to LSI
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == TI1_RMP::Lsi
    }
    ///TI1 is connected to LSE
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == TI1_RMP::Lse
    }
    ///TI1 is connected to RTC wake-up interrupt
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == TI1_RMP::Rtc
    }
}
///Field `TI1_RMP` writer - Timer 17 input 1 connection
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TI1_RMP, crate::Safe>;
impl<'a, REG> TI1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TI1 is connected to GPIO
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Gpio)
    }
    ///TI1 is connected to LSI
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Lsi)
    }
    ///TI1 is connected to LSE
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Lse)
    }
    ///TI1 is connected to RTC wake-up interrupt
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Rtc)
    }
}
impl R {
    ///Bits 0:1 - Timer 17 input 1 connection
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timer 17 input 1 connection
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, OR1rs> {
        TI1_RMP_W::new(self, 0)
    }
}
/**TIM17 option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TIM17:OR1)*/
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
///`read()` method returns [`or1::R`](R) reader structure
impl crate::Readable for OR1rs {}
///`write(|w| ..)` method takes [`or1::W`](W) writer structure
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1rs {}
