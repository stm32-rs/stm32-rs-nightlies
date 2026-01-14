///Register `CCMR2` reader
pub type R = crate::R<CCMR2rs>;
///Register `CCMR2` writer
pub type W = crate::W<CCMR2rs>;
///Field `CC3SEL` reader - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode.
pub type CC3SEL_R = crate::BitReader;
///Field `CC3SEL` writer - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode.
pub type CC3SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not.
pub type CC3E_R = crate::BitReader;
///Field `CC3E` writer - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not.
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations.
pub type CC3P_R = crate::FieldReader;
///Field `CC3P` writer - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations.
pub type CC3P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3PSC` reader - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3).
pub type IC3PSC_R = crate::FieldReader;
///Field `IC3PSC` writer - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3).
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3F` reader - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
pub type IC3F_R = crate::FieldReader;
///Field `IC3F` writer - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CC4SEL` reader - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode.
pub type CC4SEL_R = crate::BitReader;
///Field `CC4SEL` writer - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode.
pub type CC4SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not.
pub type CC4E_R = crate::BitReader;
///Field `CC4E` writer - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not.
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations.
pub type CC4P_R = crate::FieldReader;
///Field `CC4P` writer - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations.
pub type CC4P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4PSC` reader - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4).
pub type IC4PSC_R = crate::FieldReader;
///Field `IC4PSC` writer - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4).
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4F` reader - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
pub type IC4F_R = crate::FieldReader;
///Field `IC4F` writer - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode.
    #[inline(always)]
    pub fn cc3sel(&self) -> CC3SEL_R {
        CC3SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not.
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations.
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 8:9 - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3).
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 16 - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode.
    #[inline(always)]
    pub fn cc4sel(&self) -> CC4SEL_R {
        CC4SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not.
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations.
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 24:25 - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4).
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2")
            .field("cc3sel", &self.cc3sel())
            .field("cc3e", &self.cc3e())
            .field("cc3p", &self.cc3p())
            .field("ic3psc", &self.ic3psc())
            .field("ic3f", &self.ic3f())
            .field("cc4sel", &self.cc4sel())
            .field("cc4e", &self.cc4e())
            .field("cc4p", &self.cc4p())
            .field("ic4psc", &self.ic4psc())
            .field("ic4f", &self.ic4f())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode.
    #[inline(always)]
    pub fn cc3sel(&mut self) -> CC3SEL_W<'_, CCMR2rs> {
        CC3SEL_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not.
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<'_, CCMR2rs> {
        CC3E_W::new(self, 1)
    }
    ///Bits 2:3 - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations.
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<'_, CCMR2rs> {
        CC3P_W::new(self, 2)
    }
    ///Bits 8:9 - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3).
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<'_, CCMR2rs> {
        IC3PSC_W::new(self, 8)
    }
    ///Bits 12:13 - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<'_, CCMR2rs> {
        IC3F_W::new(self, 12)
    }
    ///Bit 16 - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode.
    #[inline(always)]
    pub fn cc4sel(&mut self) -> CC4SEL_W<'_, CCMR2rs> {
        CC4SEL_W::new(self, 16)
    }
    ///Bit 17 - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not.
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<'_, CCMR2rs> {
        CC4E_W::new(self, 17)
    }
    ///Bits 18:19 - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations.
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<'_, CCMR2rs> {
        CC4P_W::new(self, 18)
    }
    ///Bits 24:25 - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4).
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<'_, CCMR2rs> {
        IC4PSC_W::new(self, 24)
    }
    ///Bits 28:29 - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<'_, CCMR2rs> {
        IC4F_W::new(self, 28)
    }
}
/**LPTIM capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`ccmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM1:CCMR2)*/
pub struct CCMR2rs;
impl crate::RegisterSpec for CCMR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2::R`](R) reader structure
impl crate::Readable for CCMR2rs {}
///`write(|w| ..)` method takes [`ccmr2::W`](W) writer structure
impl crate::Writable for CCMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2 to value 0
impl crate::Resettable for CCMR2rs {}
