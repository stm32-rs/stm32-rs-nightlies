///Register `AFC0_CONFIG` reader
pub type R = crate::R<AFC0_CONFIGrs>;
///Register `AFC0_CONFIG` writer
pub type W = crate::W<AFC0_CONFIGrs>;
///Field `AFC_SLOW_GAIN_LOG2` reader - AFC loop gain in slow mode (2's log)
pub type AFC_SLOW_GAIN_LOG2_R = crate::FieldReader;
///Field `AFC_SLOW_GAIN_LOG2` writer - AFC loop gain in slow mode (2's log)
pub type AFC_SLOW_GAIN_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFC_FAST_GAIN_LOG2` reader - AFC loop gain in fast mode (2's log)
pub type AFC_FAST_GAIN_LOG2_R = crate::FieldReader;
///Field `AFC_FAST_GAIN_LOG2` writer - AFC loop gain in fast mode (2's log)
pub type AFC_FAST_GAIN_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - AFC loop gain in slow mode (2's log)
    #[inline(always)]
    pub fn afc_slow_gain_log2(&self) -> AFC_SLOW_GAIN_LOG2_R {
        AFC_SLOW_GAIN_LOG2_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AFC loop gain in fast mode (2's log)
    #[inline(always)]
    pub fn afc_fast_gain_log2(&self) -> AFC_FAST_GAIN_LOG2_R {
        AFC_FAST_GAIN_LOG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFC0_CONFIG")
            .field("afc_slow_gain_log2", &self.afc_slow_gain_log2())
            .field("afc_fast_gain_log2", &self.afc_fast_gain_log2())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AFC loop gain in slow mode (2's log)
    #[inline(always)]
    pub fn afc_slow_gain_log2(&mut self) -> AFC_SLOW_GAIN_LOG2_W<'_, AFC0_CONFIGrs> {
        AFC_SLOW_GAIN_LOG2_W::new(self, 0)
    }
    ///Bits 4:7 - AFC loop gain in fast mode (2's log)
    #[inline(always)]
    pub fn afc_fast_gain_log2(&mut self) -> AFC_FAST_GAIN_LOG2_W<'_, AFC0_CONFIGrs> {
        AFC_FAST_GAIN_LOG2_W::new(self, 4)
    }
}
/**AFC0_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc0_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc0_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC0_CONFIG)*/
pub struct AFC0_CONFIGrs;
impl crate::RegisterSpec for AFC0_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`afc0_config::R`](R) reader structure
impl crate::Readable for AFC0_CONFIGrs {}
///`write(|w| ..)` method takes [`afc0_config::W`](W) writer structure
impl crate::Writable for AFC0_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFC0_CONFIG to value 0x25
impl crate::Resettable for AFC0_CONFIGrs {
    const RESET_VALUE: u32 = 0x25;
}
