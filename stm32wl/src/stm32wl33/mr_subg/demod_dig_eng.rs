///Register `DEMOD_DIG_ENG` reader
pub type R = crate::R<DEMOD_DIG_ENGrs>;
///Register `DEMOD_DIG_ENG` writer
pub type W = crate::W<DEMOD_DIG_ENGrs>;
///Field `RX_BLANKING_LENGTH` reader - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:
pub type RX_BLANKING_LENGTH_R = crate::FieldReader;
///Field `RX_BLANKING_LENGTH` writer - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:
pub type RX_BLANKING_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:
    #[inline(always)]
    pub fn rx_blanking_length(&self) -> RX_BLANKING_LENGTH_R {
        RX_BLANKING_LENGTH_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEMOD_DIG_ENG")
            .field("rx_blanking_length", &self.rx_blanking_length())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:
    #[inline(always)]
    pub fn rx_blanking_length(&mut self) -> RX_BLANKING_LENGTH_W<'_, DEMOD_DIG_ENGrs> {
        RX_BLANKING_LENGTH_W::new(self, 0)
    }
}
/**DEMOD_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`demod_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demod_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:DEMOD_DIG_ENG)*/
pub struct DEMOD_DIG_ENGrs;
impl crate::RegisterSpec for DEMOD_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`demod_dig_eng::R`](R) reader structure
impl crate::Readable for DEMOD_DIG_ENGrs {}
///`write(|w| ..)` method takes [`demod_dig_eng::W`](W) writer structure
impl crate::Writable for DEMOD_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEMOD_DIG_ENG to value 0x03
impl crate::Resettable for DEMOD_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x03;
}
