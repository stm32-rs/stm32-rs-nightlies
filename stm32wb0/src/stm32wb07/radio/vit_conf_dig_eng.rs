///Register `VIT_CONF_DIG_ENG` reader
pub type R = crate::R<VIT_CONF_DIG_ENGrs>;
///Register `VIT_CONF_DIG_ENG` writer
pub type W = crate::W<VIT_CONF_DIG_ENGrs>;
///Field `VIT_EN` reader - Viterbi enable
pub type VIT_EN_R = crate::BitReader;
///Field `VIT_EN` writer - Viterbi enable
pub type VIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPARE` reader - spare
pub type SPARE_R = crate::FieldReader;
///Field `SPARE` writer - spare
pub type SPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Viterbi enable
    #[inline(always)]
    pub fn vit_en(&self) -> VIT_EN_R {
        VIT_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:7 - spare
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VIT_CONF_DIG_ENG")
            .field("vit_en", &self.vit_en())
            .field("spare", &self.spare())
            .finish()
    }
}
impl W {
    ///Bit 0 - Viterbi enable
    #[inline(always)]
    pub fn vit_en(&mut self) -> VIT_EN_W<'_, VIT_CONF_DIG_ENGrs> {
        VIT_EN_W::new(self, 0)
    }
    ///Bits 2:7 - spare
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W<'_, VIT_CONF_DIG_ENGrs> {
        SPARE_W::new(self, 2)
    }
}
/**VIT_CONF_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`vit_conf_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vit_conf_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:VIT_CONF_DIG_ENG)*/
pub struct VIT_CONF_DIG_ENGrs;
impl crate::RegisterSpec for VIT_CONF_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`vit_conf_dig_eng::R`](R) reader structure
impl crate::Readable for VIT_CONF_DIG_ENGrs {}
///`write(|w| ..)` method takes [`vit_conf_dig_eng::W`](W) writer structure
impl crate::Writable for VIT_CONF_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VIT_CONF_DIG_ENG to value 0
impl crate::Resettable for VIT_CONF_DIG_ENGrs {}
