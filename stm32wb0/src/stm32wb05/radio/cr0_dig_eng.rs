///Register `CR0_DIG_ENG` reader
pub type R = crate::R<CR0_DIG_ENGrs>;
///Register `CR0_DIG_ENG` writer
pub type W = crate::W<CR0_DIG_ENGrs>;
///Field `CR_GAIN_AFTER` reader - Set the gain of the clock recovery loop before Access Address detection to the value
pub type CR_GAIN_AFTER_R = crate::FieldReader;
///Field `CR_GAIN_AFTER` writer - Set the gain of the clock recovery loop before Access Address detection to the value
pub type CR_GAIN_AFTER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CR_GAIN_BEFORE` reader - Set the gain of the clock recovery loop before Access Address detection to the value
pub type CR_GAIN_BEFORE_R = crate::FieldReader;
///Field `CR_GAIN_BEFORE` writer - Set the gain of the clock recovery loop before Access Address detection to the value
pub type CR_GAIN_BEFORE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Set the gain of the clock recovery loop before Access Address detection to the value
    #[inline(always)]
    pub fn cr_gain_after(&self) -> CR_GAIN_AFTER_R {
        CR_GAIN_AFTER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Set the gain of the clock recovery loop before Access Address detection to the value
    #[inline(always)]
    pub fn cr_gain_before(&self) -> CR_GAIN_BEFORE_R {
        CR_GAIN_BEFORE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0_DIG_ENG")
            .field("cr_gain_after", &self.cr_gain_after())
            .field("cr_gain_before", &self.cr_gain_before())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Set the gain of the clock recovery loop before Access Address detection to the value
    #[inline(always)]
    pub fn cr_gain_after(&mut self) -> CR_GAIN_AFTER_W<'_, CR0_DIG_ENGrs> {
        CR_GAIN_AFTER_W::new(self, 0)
    }
    ///Bits 4:7 - Set the gain of the clock recovery loop before Access Address detection to the value
    #[inline(always)]
    pub fn cr_gain_before(&mut self) -> CR_GAIN_BEFORE_W<'_, CR0_DIG_ENGrs> {
        CR_GAIN_BEFORE_W::new(self, 4)
    }
}
/**CR0_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`cr0_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:CR0_DIG_ENG)*/
pub struct CR0_DIG_ENGrs;
impl crate::RegisterSpec for CR0_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`cr0_dig_eng::R`](R) reader structure
impl crate::Readable for CR0_DIG_ENGrs {}
///`write(|w| ..)` method takes [`cr0_dig_eng::W`](W) writer structure
impl crate::Writable for CR0_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR0_DIG_ENG to value 0x44
impl crate::Resettable for CR0_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x44;
}
