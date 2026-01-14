///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `DUAL` reader - DUAL
pub type DUAL_R = crate::FieldReader;
///Field `DUAL` writer - DUAL
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DELAY` reader - DELAY
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - DELAY
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMACFG` reader - DMACFG
pub type DMACFG_R = crate::BitReader;
///Field `DMACFG` writer - DMACFG
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDMA` reader - MDMA
pub type MDMA_R = crate::FieldReader;
///Field `MDMA` writer - MDMA
pub type MDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader;
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VREFEN` reader - VREFINT enable
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - VREFINT enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH17SEL` reader - CH17SEL
pub type CH17SEL_R = crate::BitReader;
///Field `CH17SEL` writer - CH17SEL
pub type CH17SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH18SEL` reader - CH18SEL
pub type CH18SEL_R = crate::BitReader;
///Field `CH18SEL` writer - CH18SEL
pub type CH18SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - DUAL
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:10 - DELAY
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 13 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - MDMA
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
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
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CH17SEL
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CH18SEL
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("ckmode", &self.ckmode())
            .field("presc", &self.presc())
            .field("vrefen", &self.vrefen())
            .field("ch17sel", &self.ch17sel())
            .field("ch18sel", &self.ch18sel())
            .field("mdma", &self.mdma())
            .field("dmacfg", &self.dmacfg())
            .field("delay", &self.delay())
            .field("dual", &self.dual())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DUAL
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<'_, CCRrs> {
        DUAL_W::new(self, 0)
    }
    ///Bits 8:10 - DELAY
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<'_, CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bit 13 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CCRrs> {
        DMACFG_W::new(self, 13)
    }
    ///Bits 14:15 - MDMA
    #[inline(always)]
    pub fn mdma(&mut self) -> MDMA_W<'_, CCRrs> {
        MDMA_W::new(self, 14)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, CCRrs> {
        CKMODE_W::new(self, 16)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - CH17SEL
    #[inline(always)]
    pub fn ch17sel(&mut self) -> CH17SEL_W<'_, CCRrs> {
        CH17SEL_W::new(self, 23)
    }
    ///Bit 24 - CH18SEL
    #[inline(always)]
    pub fn ch18sel(&mut self) -> CH18SEL_W<'_, CCRrs> {
        CH18SEL_W::new(self, 24)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC_common:CCR)*/
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
