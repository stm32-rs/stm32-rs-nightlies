///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader;
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VREFEN` reader - Vrefint enable
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - Vrefint enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH17SEL` reader - CH17 selection (temperature)
pub type CH17SEL_R = crate::BitReader;
///Field `CH17SEL` writer - CH17 selection (temperature)
pub type CH17SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH18SEL` reader - CH18 selection (Vbat)
pub type CH18SEL_R = crate::BitReader;
///Field `CH18SEL` writer - CH18 selection (Vbat)
pub type CH18SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - Vrefint enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CH17 selection (temperature)
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CH18 selection (Vbat)
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("ch18sel", &self.ch18sel())
            .field("ch17sel", &self.ch17sel())
            .field("vrefen", &self.vrefen())
            .field("presc", &self.presc())
            .field("ckmode", &self.ckmode())
            .finish()
    }
}
impl W {
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<CCRrs> {
        CKMODE_W::new(self, 16)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - Vrefint enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - CH17 selection (temperature)
    #[inline(always)]
    pub fn ch17sel(&mut self) -> CH17SEL_W<CCRrs> {
        CH17SEL_W::new(self, 23)
    }
    ///Bit 24 - CH18 selection (Vbat)
    #[inline(always)]
    pub fn ch18sel(&mut self) -> CH18SEL_W<CCRrs> {
        CH18SEL_W::new(self, 24)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC_Common:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
