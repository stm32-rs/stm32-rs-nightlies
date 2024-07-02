///Register `ADC_ISR` reader
pub type R = crate::R<ADC_ISRrs>;
///Register `ADC_ISR` writer
pub type W = crate::W<ADC_ISRrs>;
///Field `ADRDY` reader - ADRDY
pub type ADRDY_R = crate::BitReader;
///Field `ADRDY` writer - ADRDY
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMP` reader - EOSMP
pub type EOSMP_R = crate::BitReader;
///Field `EOSMP` writer - EOSMP
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOC` reader - EOC
pub type EOC_R = crate::BitReader;
///Field `EOC` writer - EOC
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOS` reader - EOS
pub type EOS_R = crate::BitReader;
///Field `EOS` writer - EOS
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader;
///Field `OVR` writer - OVR
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1` reader - AWD1
pub type AWD1_R = crate::BitReader;
///Field `AWD1` writer - AWD1
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2` reader - AWD2
pub type AWD2_R = crate::BitReader;
///Field `AWD2` writer - AWD2
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3` reader - AWD3
pub type AWD3_R = crate::BitReader;
///Field `AWD3` writer - AWD3
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCAL` reader - EOCAL
pub type EOCAL_R = crate::BitReader;
///Field `EOCAL` writer - EOCAL
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDORDY` reader - LDORDY
pub type LDORDY_R = crate::BitReader;
///Field `LDORDY` writer - LDORDY
pub type LDORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADRDY
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - AWD1
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD2
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - EOCAL
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LDORDY
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_ISR")
            .field("ldordy", &self.ldordy())
            .field("eocal", &self.eocal())
            .field("awd3", &self.awd3())
            .field("awd2", &self.awd2())
            .field("awd1", &self.awd1())
            .field("ovr", &self.ovr())
            .field("eos", &self.eos())
            .field("eoc", &self.eoc())
            .field("eosmp", &self.eosmp())
            .field("adrdy", &self.adrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDY
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ADC_ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ADC_ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ADC_ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ADC_ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ADC_ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 7 - AWD1
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ADC_ISRrs> {
        AWD1_W::new(self, 7)
    }
    ///Bit 8 - AWD2
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ADC_ISRrs> {
        AWD2_W::new(self, 8)
    }
    ///Bit 9 - AWD3
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ADC_ISRrs> {
        AWD3_W::new(self, 9)
    }
    ///Bit 11 - EOCAL
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<ADC_ISRrs> {
        EOCAL_W::new(self, 11)
    }
    ///Bit 12 - LDORDY
    #[inline(always)]
    #[must_use]
    pub fn ldordy(&mut self) -> LDORDY_W<ADC_ISRrs> {
        LDORDY_W::new(self, 12)
    }
}
/**ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`adc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC4:ADC_ISR)*/
pub struct ADC_ISRrs;
impl crate::RegisterSpec for ADC_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_isr::R`](R) reader structure
impl crate::Readable for ADC_ISRrs {}
///`write(|w| ..)` method takes [`adc_isr::W`](W) writer structure
impl crate::Writable for ADC_ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_ISR to value 0
impl crate::Resettable for ADC_ISRrs {
    const RESET_VALUE: u32 = 0;
}
