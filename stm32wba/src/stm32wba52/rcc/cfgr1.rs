///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `SW` reader - system clock switch Set and cleared by software to select system clock source (SYSCLK). Cleared by hardware when entering Stop and Standby modes When selecting HSE32 directly or indirectly as system clock and HSE32 oscillator clock security fails, cleared by hardware.
pub type SW_R = crate::FieldReader;
///Field `SW` writer - system clock switch Set and cleared by software to select system clock source (SYSCLK). Cleared by hardware when entering Stop and Standby modes When selecting HSE32 directly or indirectly as system clock and HSE32 oscillator clock security fails, cleared by hardware.
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWS` reader - system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock.
pub type SWS_R = crate::FieldReader;
///Field `MCOSEL` reader - microcontroller clock output Set and cleared by software. others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - microcontroller clock output Set and cleared by software. others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCOPRE` reader - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. others: not allowed
pub type MCOPRE_R = crate::FieldReader;
///Field `MCOPRE` writer - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. others: not allowed
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - system clock switch Set and cleared by software to select system clock source (SYSCLK). Cleared by hardware when entering Stop and Standby modes When selecting HSE32 directly or indirectly as system clock and HSE32 oscillator clock security fails, cleared by hardware.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock.
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 24:27 - microcontroller clock output Set and cleared by software. others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. others: not allowed
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - system clock switch Set and cleared by software to select system clock source (SYSCLK). Cleared by hardware when entering Stop and Standby modes When selecting HSE32 directly or indirectly as system clock and HSE32 oscillator clock security fails, cleared by hardware.
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CFGR1rs> {
        SW_W::new(self, 0)
    }
    ///Bits 24:27 - microcontroller clock output Set and cleared by software. others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, CFGR1rs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:30 - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. others: not allowed
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<'_, CFGR1rs> {
        MCOPRE_W::new(self, 28)
    }
}
/**RCC clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
