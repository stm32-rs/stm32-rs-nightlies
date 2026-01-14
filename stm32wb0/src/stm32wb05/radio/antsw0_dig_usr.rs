///Register `ANTSW0_DIG_USR` reader
pub type R = crate::R<ANTSW0_DIG_USRrs>;
///Register `ANTSW0_DIG_USR` writer
pub type W = crate::W<ANTSW0_DIG_USRrs>;
///Field `RX_TIME_TO_SAMPLE` reader - specifies the exact timing of the first I/Q sampling in the reference period.
pub type RX_TIME_TO_SAMPLE_R = crate::FieldReader;
///Field `RX_TIME_TO_SAMPLE` writer - specifies the exact timing of the first I/Q sampling in the reference period.
pub type RX_TIME_TO_SAMPLE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - specifies the exact timing of the first I/Q sampling in the reference period.
    #[inline(always)]
    pub fn rx_time_to_sample(&self) -> RX_TIME_TO_SAMPLE_R {
        RX_TIME_TO_SAMPLE_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANTSW0_DIG_USR")
            .field("rx_time_to_sample", &self.rx_time_to_sample())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - specifies the exact timing of the first I/Q sampling in the reference period.
    #[inline(always)]
    pub fn rx_time_to_sample(&mut self) -> RX_TIME_TO_SAMPLE_W<'_, ANTSW0_DIG_USRrs> {
        RX_TIME_TO_SAMPLE_W::new(self, 0)
    }
}
/**ANTSW0_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw0_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw0_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:ANTSW0_DIG_USR)*/
pub struct ANTSW0_DIG_USRrs;
impl crate::RegisterSpec for ANTSW0_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`antsw0_dig_usr::R`](R) reader structure
impl crate::Readable for ANTSW0_DIG_USRrs {}
///`write(|w| ..)` method takes [`antsw0_dig_usr::W`](W) writer structure
impl crate::Writable for ANTSW0_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANTSW0_DIG_USR to value 0x1c
impl crate::Resettable for ANTSW0_DIG_USRrs {
    const RESET_VALUE: u32 = 0x1c;
}
