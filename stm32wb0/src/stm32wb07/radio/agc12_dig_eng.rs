///Register `AGC12_DIG_ENG` reader
pub type R = crate::R<AGC12_DIG_ENGrs>;
///Register `AGC12_DIG_ENG` writer
pub type W = crate::W<AGC12_DIG_ENGrs>;
///Field `ATT_IF_2` reader - Attenuation at IF Level for the AGC step 2
pub type ATT_IF_2_R = crate::FieldReader;
///Field `ATT_IF_2` writer - Attenuation at IF Level for the AGC step 2
pub type ATT_IF_2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATT_LNA_2` reader - Attenuation at LNA Level for the AGC step 2
pub type ATT_LNA_2_R = crate::BitReader;
///Field `ATT_LNA_2` writer - Attenuation at LNA Level for the AGC step 2
pub type ATT_LNA_2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATT_ANT_2` reader - Attenuation at Antenna Level for the AGC step 2
pub type ATT_ANT_2_R = crate::FieldReader;
///Field `ATT_ANT_2` writer - Attenuation at Antenna Level for the AGC step 2
pub type ATT_ANT_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Attenuation at IF Level for the AGC step 2
    #[inline(always)]
    pub fn att_if_2(&self) -> ATT_IF_2_R {
        ATT_IF_2_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Attenuation at LNA Level for the AGC step 2
    #[inline(always)]
    pub fn att_lna_2(&self) -> ATT_LNA_2_R {
        ATT_LNA_2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Attenuation at Antenna Level for the AGC step 2
    #[inline(always)]
    pub fn att_ant_2(&self) -> ATT_ANT_2_R {
        ATT_ANT_2_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC12_DIG_ENG")
            .field("att_if_2", &self.att_if_2())
            .field("att_lna_2", &self.att_lna_2())
            .field("att_ant_2", &self.att_ant_2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Attenuation at IF Level for the AGC step 2
    #[inline(always)]
    pub fn att_if_2(&mut self) -> ATT_IF_2_W<'_, AGC12_DIG_ENGrs> {
        ATT_IF_2_W::new(self, 0)
    }
    ///Bit 3 - Attenuation at LNA Level for the AGC step 2
    #[inline(always)]
    pub fn att_lna_2(&mut self) -> ATT_LNA_2_W<'_, AGC12_DIG_ENGrs> {
        ATT_LNA_2_W::new(self, 3)
    }
    ///Bits 4:5 - Attenuation at Antenna Level for the AGC step 2
    #[inline(always)]
    pub fn att_ant_2(&mut self) -> ATT_ANT_2_W<'_, AGC12_DIG_ENGrs> {
        ATT_ANT_2_W::new(self, 4)
    }
}
/**AGC12_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc12_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc12_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:AGC12_DIG_ENG)*/
pub struct AGC12_DIG_ENGrs;
impl crate::RegisterSpec for AGC12_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`agc12_dig_eng::R`](R) reader structure
impl crate::Readable for AGC12_DIG_ENGrs {}
///`write(|w| ..)` method takes [`agc12_dig_eng::W`](W) writer structure
impl crate::Writable for AGC12_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC12_DIG_ENG to value 0x20
impl crate::Resettable for AGC12_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x20;
}
