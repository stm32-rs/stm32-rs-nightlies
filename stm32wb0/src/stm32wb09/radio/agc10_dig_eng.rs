///Register `AGC10_DIG_ENG` reader
pub type R = crate::R<AGC10_DIG_ENGrs>;
///Register `AGC10_DIG_ENG` writer
pub type W = crate::W<AGC10_DIG_ENGrs>;
///Field `ATT_IF_0` reader - Attenuation at IF Level for the AGC step 0:
pub type ATT_IF_0_R = crate::FieldReader;
///Field `ATT_IF_0` writer - Attenuation at IF Level for the AGC step 0:
pub type ATT_IF_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATT_LNA_0` reader - Attenuation at LNA Level for the AGC step 0:
pub type ATT_LNA_0_R = crate::BitReader;
///Field `ATT_LNA_0` writer - Attenuation at LNA Level for the AGC step 0:
pub type ATT_LNA_0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATT_ANT_0` reader - Attenuation at Antenna Level for the AGC step 0:
pub type ATT_ANT_0_R = crate::FieldReader;
///Field `ATT_ANT_0` writer - Attenuation at Antenna Level for the AGC step 0:
pub type ATT_ANT_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Attenuation at IF Level for the AGC step 0:
    #[inline(always)]
    pub fn att_if_0(&self) -> ATT_IF_0_R {
        ATT_IF_0_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Attenuation at LNA Level for the AGC step 0:
    #[inline(always)]
    pub fn att_lna_0(&self) -> ATT_LNA_0_R {
        ATT_LNA_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Attenuation at Antenna Level for the AGC step 0:
    #[inline(always)]
    pub fn att_ant_0(&self) -> ATT_ANT_0_R {
        ATT_ANT_0_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC10_DIG_ENG")
            .field("att_if_0", &self.att_if_0())
            .field("att_lna_0", &self.att_lna_0())
            .field("att_ant_0", &self.att_ant_0())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Attenuation at IF Level for the AGC step 0:
    #[inline(always)]
    pub fn att_if_0(&mut self) -> ATT_IF_0_W<'_, AGC10_DIG_ENGrs> {
        ATT_IF_0_W::new(self, 0)
    }
    ///Bit 3 - Attenuation at LNA Level for the AGC step 0:
    #[inline(always)]
    pub fn att_lna_0(&mut self) -> ATT_LNA_0_W<'_, AGC10_DIG_ENGrs> {
        ATT_LNA_0_W::new(self, 3)
    }
    ///Bits 4:5 - Attenuation at Antenna Level for the AGC step 0:
    #[inline(always)]
    pub fn att_ant_0(&mut self) -> ATT_ANT_0_W<'_, AGC10_DIG_ENGrs> {
        ATT_ANT_0_W::new(self, 4)
    }
}
/**AGC10_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc10_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc10_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC10_DIG_ENG)*/
pub struct AGC10_DIG_ENGrs;
impl crate::RegisterSpec for AGC10_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`agc10_dig_eng::R`](R) reader structure
impl crate::Readable for AGC10_DIG_ENGrs {}
///`write(|w| ..)` method takes [`agc10_dig_eng::W`](W) writer structure
impl crate::Writable for AGC10_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC10_DIG_ENG to value 0
impl crate::Resettable for AGC10_DIG_ENGrs {}
