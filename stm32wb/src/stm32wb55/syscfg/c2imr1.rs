#[doc = "Register `C2IMR1` reader"]
pub type R = crate::R<C2IMR1rs>;
#[doc = "Register `C2IMR1` writer"]
pub type W = crate::W<C2IMR1rs>;
#[doc = "Field `RTCSTAMP` reader - Peripheral RTCSTAMP interrupt mask to CPU2"]
pub type RTCSTAMP_R = crate::BitReader;
#[doc = "Field `RTCSTAMP` writer - Peripheral RTCSTAMP interrupt mask to CPU2"]
pub type RTCSTAMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCWKUP` reader - Peripheral RTCWKUP interrupt mask to CPU2"]
pub type RTCWKUP_R = crate::BitReader;
#[doc = "Field `RTCWKUP` writer - Peripheral RTCWKUP interrupt mask to CPU2"]
pub type RTCWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCALARM` reader - Peripheral RTCALARM interrupt mask to CPU2"]
pub type RTCALARM_R = crate::BitReader;
#[doc = "Field `RTCALARM` writer - Peripheral RTCALARM interrupt mask to CPU2"]
pub type RTCALARM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCC` reader - Peripheral RCC interrupt mask to CPU2"]
pub type RCC_R = crate::BitReader;
#[doc = "Field `RCC` writer - Peripheral RCC interrupt mask to CPU2"]
pub type RCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH` reader - Peripheral FLASH interrupt mask to CPU2"]
pub type FLASH_R = crate::BitReader;
#[doc = "Field `FLASH` writer - Peripheral FLASH interrupt mask to CPU2"]
pub type FLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA` reader - Peripheral PKA interrupt mask to CPU2"]
pub type PKA_R = crate::BitReader;
#[doc = "Field `PKA` writer - Peripheral PKA interrupt mask to CPU2"]
pub type PKA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG` reader - Peripheral RNG interrupt mask to CPU2"]
pub type RNG_R = crate::BitReader;
#[doc = "Field `RNG` writer - Peripheral RNG interrupt mask to CPU2"]
pub type RNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES1` reader - Peripheral AES1 interrupt mask to CPU2"]
pub type AES1_R = crate::BitReader;
#[doc = "Field `AES1` writer - Peripheral AES1 interrupt mask to CPU2"]
pub type AES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - Peripheral COMP interrupt mask to CPU2"]
pub type COMP_R = crate::BitReader;
#[doc = "Field `COMP` writer - Peripheral COMP interrupt mask to CPU2"]
pub type COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC` reader - Peripheral ADC interrupt mask to CPU2"]
pub type ADC_R = crate::BitReader;
#[doc = "Field `ADC` writer - Peripheral ADC interrupt mask to CPU2"]
pub type ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcstamp(&self) -> RTCSTAMP_R {
        RTCSTAMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcwkup(&self) -> RTCWKUP_R {
        RTCWKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcalarm(&self) -> RTCALARM_R {
        RTCALARM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn aes1(&self) -> AES1_R {
        AES1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rtcstamp(&mut self) -> RTCSTAMP_W<C2IMR1rs> {
        RTCSTAMP_W::new(self, 0)
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rtcwkup(&mut self) -> RTCWKUP_W<C2IMR1rs> {
        RTCWKUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rtcalarm(&mut self) -> RTCALARM_W<C2IMR1rs> {
        RTCALARM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rcc(&mut self) -> RCC_W<C2IMR1rs> {
        RCC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<C2IMR1rs> {
        FLASH_W::new(self, 6)
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pka(&mut self) -> PKA_W<C2IMR1rs> {
        PKA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<C2IMR1rs> {
        RNG_W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn aes1(&mut self) -> AES1_W<C2IMR1rs> {
        AES1_W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<C2IMR1rs> {
        COMP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<C2IMR1rs> {
        ADC_W::new(self, 12)
    }
}
#[doc = "CPU2 interrupt mask register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2IMR1rs;
impl crate::RegisterSpec for C2IMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2imr1::R`](R) reader structure"]
impl crate::Readable for C2IMR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2imr1::W`](W) writer structure"]
impl crate::Writable for C2IMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1rs {
    const RESET_VALUE: u32 = 0;
}
