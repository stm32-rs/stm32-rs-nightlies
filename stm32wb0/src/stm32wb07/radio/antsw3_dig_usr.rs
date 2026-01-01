///Register `ANTSW3_DIG_USR` reader
pub type R = crate::R<ANTSW3_DIG_USRrs>;
///Register `ANTSW3_DIG_USR` writer
pub type W = crate::W<ANTSW3_DIG_USRrs>;
///Field `TX_TIME_TO_SWITCH_2M` reader - specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD).
pub type TX_TIME_TO_SWITCH_2M_R = crate::FieldReader;
///Field `TX_TIME_TO_SWITCH_2M` writer - specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD).
pub type TX_TIME_TO_SWITCH_2M_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD).
    #[inline(always)]
    pub fn tx_time_to_switch_2m(&self) -> TX_TIME_TO_SWITCH_2M_R {
        TX_TIME_TO_SWITCH_2M_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANTSW3_DIG_USR")
            .field("tx_time_to_switch_2m", &self.tx_time_to_switch_2m())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD).
    #[inline(always)]
    pub fn tx_time_to_switch_2m(&mut self) -> TX_TIME_TO_SWITCH_2M_W<'_, ANTSW3_DIG_USRrs> {
        TX_TIME_TO_SWITCH_2M_W::new(self, 0)
    }
}
/**ANTSW3_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw3_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw3_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:ANTSW3_DIG_USR)*/
pub struct ANTSW3_DIG_USRrs;
impl crate::RegisterSpec for ANTSW3_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`antsw3_dig_usr::R`](R) reader structure
impl crate::Readable for ANTSW3_DIG_USRrs {}
///`write(|w| ..)` method takes [`antsw3_dig_usr::W`](W) writer structure
impl crate::Writable for ANTSW3_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANTSW3_DIG_USR to value 0x23
impl crate::Resettable for ANTSW3_DIG_USRrs {
    const RESET_VALUE: u32 = 0x23;
}
