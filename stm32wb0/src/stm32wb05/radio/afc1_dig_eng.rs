///Register `AFC1_DIG_ENG` reader
pub type R = crate::R<AFC1_DIG_ENGrs>;
///Register `AFC1_DIG_ENG` writer
pub type W = crate::W<AFC1_DIG_ENGrs>;
///Field `AFC_DELAY_AFTER` reader - Set the decay factor of the AFC loop after Access Address detection
pub type AFC_DELAY_AFTER_R = crate::FieldReader;
///Field `AFC_DELAY_AFTER` writer - Set the decay factor of the AFC loop after Access Address detection
pub type AFC_DELAY_AFTER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFC_DELAY_BEFORE` reader - Set the decay factor of the AFC loop before Access Address detection
pub type AFC_DELAY_BEFORE_R = crate::FieldReader;
///Field `AFC_DELAY_BEFORE` writer - Set the decay factor of the AFC loop before Access Address detection
pub type AFC_DELAY_BEFORE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Set the decay factor of the AFC loop after Access Address detection
    #[inline(always)]
    pub fn afc_delay_after(&self) -> AFC_DELAY_AFTER_R {
        AFC_DELAY_AFTER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Set the decay factor of the AFC loop before Access Address detection
    #[inline(always)]
    pub fn afc_delay_before(&self) -> AFC_DELAY_BEFORE_R {
        AFC_DELAY_BEFORE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFC1_DIG_ENG")
            .field("afc_delay_after", &self.afc_delay_after())
            .field("afc_delay_before", &self.afc_delay_before())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Set the decay factor of the AFC loop after Access Address detection
    #[inline(always)]
    pub fn afc_delay_after(&mut self) -> AFC_DELAY_AFTER_W<'_, AFC1_DIG_ENGrs> {
        AFC_DELAY_AFTER_W::new(self, 0)
    }
    ///Bits 4:7 - Set the decay factor of the AFC loop before Access Address detection
    #[inline(always)]
    pub fn afc_delay_before(&mut self) -> AFC_DELAY_BEFORE_W<'_, AFC1_DIG_ENGrs> {
        AFC_DELAY_BEFORE_W::new(self, 4)
    }
}
/**AFC1_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`afc1_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc1_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:AFC1_DIG_ENG)*/
pub struct AFC1_DIG_ENGrs;
impl crate::RegisterSpec for AFC1_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`afc1_dig_eng::R`](R) reader structure
impl crate::Readable for AFC1_DIG_ENGrs {}
///`write(|w| ..)` method takes [`afc1_dig_eng::W`](W) writer structure
impl crate::Writable for AFC1_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFC1_DIG_ENG to value 0x44
impl crate::Resettable for AFC1_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x44;
}
