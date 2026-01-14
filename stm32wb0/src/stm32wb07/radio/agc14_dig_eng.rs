///Register `AGC14_DIG_ENG` reader
pub type R = crate::R<AGC14_DIG_ENGrs>;
///Register `AGC14_DIG_ENG` writer
pub type W = crate::W<AGC14_DIG_ENGrs>;
///Field `ATT_IF_4` reader - Attenuation at IF Level for the AGC step 4
pub type ATT_IF_4_R = crate::FieldReader;
///Field `ATT_IF_4` writer - Attenuation at IF Level for the AGC step 4
pub type ATT_IF_4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATT_LNA_4` reader - Attenuation at LNA Level for the AGC step 4
pub type ATT_LNA_4_R = crate::BitReader;
///Field `ATT_LNA_4` writer - Attenuation at LNA Level for the AGC step 4
pub type ATT_LNA_4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATT_ANT_4` reader - Attenuation at Antenna Level for the AGC step 4
pub type ATT_ANT_4_R = crate::FieldReader;
///Field `ATT_ANT_4` writer - Attenuation at Antenna Level for the AGC step 4
pub type ATT_ANT_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Attenuation at IF Level for the AGC step 4
    #[inline(always)]
    pub fn att_if_4(&self) -> ATT_IF_4_R {
        ATT_IF_4_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Attenuation at LNA Level for the AGC step 4
    #[inline(always)]
    pub fn att_lna_4(&self) -> ATT_LNA_4_R {
        ATT_LNA_4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Attenuation at Antenna Level for the AGC step 4
    #[inline(always)]
    pub fn att_ant_4(&self) -> ATT_ANT_4_R {
        ATT_ANT_4_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC14_DIG_ENG")
            .field("att_if_4", &self.att_if_4())
            .field("att_lna_4", &self.att_lna_4())
            .field("att_ant_4", &self.att_ant_4())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Attenuation at IF Level for the AGC step 4
    #[inline(always)]
    pub fn att_if_4(&mut self) -> ATT_IF_4_W<'_, AGC14_DIG_ENGrs> {
        ATT_IF_4_W::new(self, 0)
    }
    ///Bit 3 - Attenuation at LNA Level for the AGC step 4
    #[inline(always)]
    pub fn att_lna_4(&mut self) -> ATT_LNA_4_W<'_, AGC14_DIG_ENGrs> {
        ATT_LNA_4_W::new(self, 3)
    }
    ///Bits 4:5 - Attenuation at Antenna Level for the AGC step 4
    #[inline(always)]
    pub fn att_ant_4(&mut self) -> ATT_ANT_4_W<'_, AGC14_DIG_ENGrs> {
        ATT_ANT_4_W::new(self, 4)
    }
}
/**AGC14_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc14_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc14_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:AGC14_DIG_ENG)*/
pub struct AGC14_DIG_ENGrs;
impl crate::RegisterSpec for AGC14_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`agc14_dig_eng::R`](R) reader structure
impl crate::Readable for AGC14_DIG_ENGrs {}
///`write(|w| ..)` method takes [`agc14_dig_eng::W`](W) writer structure
impl crate::Writable for AGC14_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC14_DIG_ENG to value 0x38
impl crate::Resettable for AGC14_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x38;
}
