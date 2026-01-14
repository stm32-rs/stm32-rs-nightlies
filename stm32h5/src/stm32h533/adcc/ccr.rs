///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `DUAL` reader - Dual ADC mode selection
pub type DUAL_R = crate::FieldReader;
///Field `DUAL` writer - Dual ADC mode selection
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMACFG` reader - DMA configuration (for dual ADC mode)
pub type DMACFG_R = crate::BitReader;
///Field `DMACFG` writer - DMA configuration (for dual ADC mode)
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDMA` reader - Direct memory access mode for dual ADC mode
pub type MDMA_R = crate::FieldReader;
///Field `MDMA` writer - Direct memory access mode for dual ADC mode
pub type MDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader;
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VREFEN` reader - Vless thansub>REFINTless than/sub> enable
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - Vless thansub>REFINTless than/sub> enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN` reader - Vless thansub>SENSEless than/sub> enable
pub type TSEN_R = crate::BitReader;
///Field `TSEN` writer - Vless thansub>SENSEless than/sub> enable
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBATEN` reader - VBAT enable
pub type VBATEN_R = crate::BitReader;
///Field `VBATEN` writer - VBAT enable
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA configuration (for dual ADC mode)
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for dual ADC mode
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
    ///Bit 22 - Vless thansub>REFINTless than/sub> enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Vless thansub>SENSEless than/sub> enable
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dual", &self.dual())
            .field("delay", &self.delay())
            .field("dmacfg", &self.dmacfg())
            .field("mdma", &self.mdma())
            .field("ckmode", &self.ckmode())
            .field("presc", &self.presc())
            .field("vrefen", &self.vrefen())
            .field("tsen", &self.tsen())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<'_, CCRrs> {
        DUAL_W::new(self, 0)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<'_, CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bit 13 - DMA configuration (for dual ADC mode)
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CCRrs> {
        DMACFG_W::new(self, 13)
    }
    ///Bits 14:15 - Direct memory access mode for dual ADC mode
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
    ///Bit 22 - Vless thansub>REFINTless than/sub> enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Vless thansub>SENSEless than/sub> enable
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<'_, CCRrs> {
        TSEN_W::new(self, 23)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<'_, CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#ADCC:CCR)*/
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
