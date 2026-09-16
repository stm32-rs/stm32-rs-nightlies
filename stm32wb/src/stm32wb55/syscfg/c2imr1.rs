///Register `C2IMR1` reader
pub type R = crate::R<C2IMR1rs>;
///Register `C2IMR1` writer
pub type W = crate::W<C2IMR1rs>;
/**Peripheral RTCSTAMP interrupt mask to CPU2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCSTAMP {
    ///0: Peripheral interrupt forwarded to CPU2
    Unmasked = 0,
    ///1: Peripheral interrupt to CPU2 masked
    Masked = 1,
}
impl From<RTCSTAMP> for bool {
    #[inline(always)]
    fn from(variant: RTCSTAMP) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCSTAMP` reader - Peripheral RTCSTAMP interrupt mask to CPU2
pub type RTCSTAMP_R = crate::BitReader<RTCSTAMP>;
impl RTCSTAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCSTAMP {
        match self.bits {
            false => RTCSTAMP::Unmasked,
            true => RTCSTAMP::Masked,
        }
    }
    ///Peripheral interrupt forwarded to CPU2
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RTCSTAMP::Unmasked
    }
    ///Peripheral interrupt to CPU2 masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RTCSTAMP::Masked
    }
}
///Field `RTCSTAMP` writer - Peripheral RTCSTAMP interrupt mask to CPU2
pub type RTCSTAMP_W<'a, REG> = crate::BitWriter<'a, REG, RTCSTAMP>;
impl<'a, REG> RTCSTAMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral interrupt forwarded to CPU2
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSTAMP::Unmasked)
    }
    ///Peripheral interrupt to CPU2 masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSTAMP::Masked)
    }
}
///Field `RTCWKUP` reader - Peripheral RTCWKUP interrupt mask to CPU2
pub use RTCSTAMP_R as RTCWKUP_R;
///Field `RTCALARM` reader - Peripheral RTCALARM interrupt mask to CPU2
pub use RTCSTAMP_R as RTCALARM_R;
///Field `RCC` reader - Peripheral RCC interrupt mask to CPU2
pub use RTCSTAMP_R as RCC_R;
///Field `FLASH` reader - Peripheral FLASH interrupt mask to CPU2
pub use RTCSTAMP_R as FLASH_R;
///Field `PKA` reader - Peripheral PKA interrupt mask to CPU2
pub use RTCSTAMP_R as PKA_R;
///Field `RNG` reader - Peripheral RNG interrupt mask to CPU2
pub use RTCSTAMP_R as RNG_R;
///Field `AES1` reader - Peripheral AES1 interrupt mask to CPU2
pub use RTCSTAMP_R as AES1_R;
///Field `COMP` reader - Peripheral COMP interrupt mask to CPU2
pub use RTCSTAMP_R as COMP_R;
///Field `ADC` reader - Peripheral ADC interrupt mask to CPU2
pub use RTCSTAMP_R as ADC_R;
///Field `RTCWKUP` writer - Peripheral RTCWKUP interrupt mask to CPU2
pub use RTCSTAMP_W as RTCWKUP_W;
///Field `RTCALARM` writer - Peripheral RTCALARM interrupt mask to CPU2
pub use RTCSTAMP_W as RTCALARM_W;
///Field `RCC` writer - Peripheral RCC interrupt mask to CPU2
pub use RTCSTAMP_W as RCC_W;
///Field `FLASH` writer - Peripheral FLASH interrupt mask to CPU2
pub use RTCSTAMP_W as FLASH_W;
///Field `PKA` writer - Peripheral PKA interrupt mask to CPU2
pub use RTCSTAMP_W as PKA_W;
///Field `RNG` writer - Peripheral RNG interrupt mask to CPU2
pub use RTCSTAMP_W as RNG_W;
///Field `AES1` writer - Peripheral AES1 interrupt mask to CPU2
pub use RTCSTAMP_W as AES1_W;
///Field `COMP` writer - Peripheral COMP interrupt mask to CPU2
pub use RTCSTAMP_W as COMP_W;
///Field `ADC` writer - Peripheral ADC interrupt mask to CPU2
pub use RTCSTAMP_W as ADC_W;
impl R {
    ///Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2
    #[inline(always)]
    pub fn rtcstamp(&self) -> RTCSTAMP_R {
        RTCSTAMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2
    #[inline(always)]
    pub fn rtcwkup(&self) -> RTCWKUP_R {
        RTCWKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Peripheral RTCALARM interrupt mask to CPU2
    #[inline(always)]
    pub fn rtcalarm(&self) -> RTCALARM_R {
        RTCALARM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral RCC interrupt mask to CPU2
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral FLASH interrupt mask to CPU2
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Peripheral PKA interrupt mask to CPU2
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Peripheral RNG interrupt mask to CPU2
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Peripheral AES1 interrupt mask to CPU2
    #[inline(always)]
    pub fn aes1(&self) -> AES1_R {
        AES1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Peripheral COMP interrupt mask to CPU2
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Peripheral ADC interrupt mask to CPU2
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2IMR1")
            .field("rtcstamp", &self.rtcstamp())
            .field("rtcwkup", &self.rtcwkup())
            .field("rtcalarm", &self.rtcalarm())
            .field("rcc", &self.rcc())
            .field("flash", &self.flash())
            .field("pka", &self.pka())
            .field("rng", &self.rng())
            .field("aes1", &self.aes1())
            .field("comp", &self.comp())
            .field("adc", &self.adc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2
    #[inline(always)]
    pub fn rtcstamp(&mut self) -> RTCSTAMP_W<'_, C2IMR1rs> {
        RTCSTAMP_W::new(self, 0)
    }
    ///Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2
    #[inline(always)]
    pub fn rtcwkup(&mut self) -> RTCWKUP_W<'_, C2IMR1rs> {
        RTCWKUP_W::new(self, 3)
    }
    ///Bit 4 - Peripheral RTCALARM interrupt mask to CPU2
    #[inline(always)]
    pub fn rtcalarm(&mut self) -> RTCALARM_W<'_, C2IMR1rs> {
        RTCALARM_W::new(self, 4)
    }
    ///Bit 5 - Peripheral RCC interrupt mask to CPU2
    #[inline(always)]
    pub fn rcc(&mut self) -> RCC_W<'_, C2IMR1rs> {
        RCC_W::new(self, 5)
    }
    ///Bit 6 - Peripheral FLASH interrupt mask to CPU2
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W<'_, C2IMR1rs> {
        FLASH_W::new(self, 6)
    }
    ///Bit 8 - Peripheral PKA interrupt mask to CPU2
    #[inline(always)]
    pub fn pka(&mut self) -> PKA_W<'_, C2IMR1rs> {
        PKA_W::new(self, 8)
    }
    ///Bit 9 - Peripheral RNG interrupt mask to CPU2
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W<'_, C2IMR1rs> {
        RNG_W::new(self, 9)
    }
    ///Bit 10 - Peripheral AES1 interrupt mask to CPU2
    #[inline(always)]
    pub fn aes1(&mut self) -> AES1_W<'_, C2IMR1rs> {
        AES1_W::new(self, 10)
    }
    ///Bit 11 - Peripheral COMP interrupt mask to CPU2
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<'_, C2IMR1rs> {
        COMP_W::new(self, 11)
    }
    ///Bit 12 - Peripheral ADC interrupt mask to CPU2
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W<'_, C2IMR1rs> {
        ADC_W::new(self, 12)
    }
}
/**CPU2 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`c2imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:C2IMR1)*/
pub struct C2IMR1rs;
impl crate::RegisterSpec for C2IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2imr1::R`](R) reader structure
impl crate::Readable for C2IMR1rs {}
///`write(|w| ..)` method takes [`c2imr1::W`](W) writer structure
impl crate::Writable for C2IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2IMR1 to value 0
impl crate::Resettable for C2IMR1rs {}
