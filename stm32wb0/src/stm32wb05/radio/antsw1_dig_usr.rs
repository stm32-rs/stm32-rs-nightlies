///Register `ANTSW1_DIG_USR` reader
pub type R = crate::R<ANTSW1_DIG_USRrs>;
///Register `ANTSW1_DIG_USR` writer
pub type W = crate::W<ANTSW1_DIG_USRrs>;
///Field `RX_TIME_TO_SWITCH` reader - specifies the exact timing of the antenna switching at receiver level (in AoA).
pub type RX_TIME_TO_SWITCH_R = crate::FieldReader;
///Field `RX_TIME_TO_SWITCH` writer - specifies the exact timing of the antenna switching at receiver level (in AoA).
pub type RX_TIME_TO_SWITCH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - specifies the exact timing of the antenna switching at receiver level (in AoA).
    #[inline(always)]
    pub fn rx_time_to_switch(&self) -> RX_TIME_TO_SWITCH_R {
        RX_TIME_TO_SWITCH_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANTSW1_DIG_USR")
            .field("rx_time_to_switch", &self.rx_time_to_switch())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - specifies the exact timing of the antenna switching at receiver level (in AoA).
    #[inline(always)]
    pub fn rx_time_to_switch(&mut self) -> RX_TIME_TO_SWITCH_W<'_, ANTSW1_DIG_USRrs> {
        RX_TIME_TO_SWITCH_W::new(self, 0)
    }
}
/**ANTSW1_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw1_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw1_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:ANTSW1_DIG_USR)*/
pub struct ANTSW1_DIG_USRrs;
impl crate::RegisterSpec for ANTSW1_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`antsw1_dig_usr::R`](R) reader structure
impl crate::Readable for ANTSW1_DIG_USRrs {}
///`write(|w| ..)` method takes [`antsw1_dig_usr::W`](W) writer structure
impl crate::Writable for ANTSW1_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANTSW1_DIG_USR to value 0x0b
impl crate::Resettable for ANTSW1_DIG_USRrs {
    const RESET_VALUE: u32 = 0x0b;
}
