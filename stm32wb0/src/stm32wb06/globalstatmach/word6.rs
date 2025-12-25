///Register `WORD6` reader
pub type R = crate::R<WORD6rs>;
///Register `WORD6` writer
pub type W = crate::W<WORD6rs>;
///Field `DefaultAntennaID` reader - Default Antenna ID corresponding to the number of the antenna used to receive/transmit:
pub type DEFAULT_ANTENNA_ID_R = crate::FieldReader;
///Field `DefaultAntennaID` writer - Default Antenna ID corresponding to the number of the antenna used to receive/transmit:
pub type DEFAULT_ANTENNA_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Default Antenna ID corresponding to the number of the antenna used to receive/transmit:
    #[inline(always)]
    pub fn default_antenna_id(&self) -> DEFAULT_ANTENNA_ID_R {
        DEFAULT_ANTENNA_ID_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD6")
            .field("default_antenna_id", &self.default_antenna_id())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Default Antenna ID corresponding to the number of the antenna used to receive/transmit:
    #[inline(always)]
    pub fn default_antenna_id(&mut self) -> DEFAULT_ANTENNA_ID_W<'_, WORD6rs> {
        DEFAULT_ANTENNA_ID_W::new(self, 0)
    }
}
/**WORD6 register

You can [`read`](crate::Reg::read) this register and get [`word6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#GLOBALSTATMACH:WORD6)*/
pub struct WORD6rs;
impl crate::RegisterSpec for WORD6rs {
    type Ux = u32;
}
///`read()` method returns [`word6::R`](R) reader structure
impl crate::Readable for WORD6rs {}
///`write(|w| ..)` method takes [`word6::W`](W) writer structure
impl crate::Writable for WORD6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD6 to value 0
impl crate::Resettable for WORD6rs {}
