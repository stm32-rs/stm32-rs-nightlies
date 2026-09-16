///Register `MC_APB3ENCLRR` reader
pub type R = crate::R<MC_APB3ENCLRRrs>;
///Register `MC_APB3ENCLRR` writer
pub type W = crate::W<MC_APB3ENCLRRrs>;
///Field `LPTIM2EN` reader - LPTIM2EN
pub type LPTIM2EN_R = crate::BitReader;
///Field `LPTIM2EN` writer - LPTIM2EN
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3EN` reader - LPTIM3EN
pub type LPTIM3EN_R = crate::BitReader;
///Field `LPTIM3EN` writer - LPTIM3EN
pub type LPTIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4EN` reader - LPTIM4EN
pub type LPTIM4EN_R = crate::BitReader;
///Field `LPTIM4EN` writer - LPTIM4EN
pub type LPTIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5EN` reader - LPTIM5EN
pub type LPTIM5EN_R = crate::BitReader;
///Field `LPTIM5EN` writer - LPTIM5EN
pub type LPTIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI4EN` reader - SAI4EN
pub type SAI4EN_R = crate::BitReader;
///Field `SAI4EN` writer - SAI4EN
pub type SAI4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGEN` reader - SYSCFGEN
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFGEN
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFEN` reader - VREFEN
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - VREFEN
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSEN` reader - DTSEN
pub type DTSEN_R = crate::BitReader;
///Field `DTSEN` writer - DTSEN
pub type DTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDPEN` reader - HDPEN
pub type HDPEN_R = crate::BitReader;
///Field `HDPEN` writer - HDPEN
pub type HDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPTIM2EN
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM3EN
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPTIM4EN
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPTIM5EN
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SAI4EN
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SYSCFGEN
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - VREFEN
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - DTSEN
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - HDPEN
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_APB3ENCLRR")
            .field("lptim2en", &self.lptim2en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim4en", &self.lptim4en())
            .field("lptim5en", &self.lptim5en())
            .field("sai4en", &self.sai4en())
            .field("syscfgen", &self.syscfgen())
            .field("vrefen", &self.vrefen())
            .field("dtsen", &self.dtsen())
            .field("hdpen", &self.hdpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM2EN
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, MC_APB3ENCLRRrs> {
        LPTIM2EN_W::new(self, 0)
    }
    ///Bit 1 - LPTIM3EN
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, MC_APB3ENCLRRrs> {
        LPTIM3EN_W::new(self, 1)
    }
    ///Bit 2 - LPTIM4EN
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<'_, MC_APB3ENCLRRrs> {
        LPTIM4EN_W::new(self, 2)
    }
    ///Bit 3 - LPTIM5EN
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<'_, MC_APB3ENCLRRrs> {
        LPTIM5EN_W::new(self, 3)
    }
    ///Bit 8 - SAI4EN
    #[inline(always)]
    pub fn sai4en(&mut self) -> SAI4EN_W<'_, MC_APB3ENCLRRrs> {
        SAI4EN_W::new(self, 8)
    }
    ///Bit 11 - SYSCFGEN
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, MC_APB3ENCLRRrs> {
        SYSCFGEN_W::new(self, 11)
    }
    ///Bit 13 - VREFEN
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, MC_APB3ENCLRRrs> {
        VREFEN_W::new(self, 13)
    }
    ///Bit 16 - DTSEN
    #[inline(always)]
    pub fn dtsen(&mut self) -> DTSEN_W<'_, MC_APB3ENCLRRrs> {
        DTSEN_W::new(self, 16)
    }
    ///Bit 20 - HDPEN
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W<'_, MC_APB3ENCLRRrs> {
        HDPEN_W::new(self, 20)
    }
}
/**This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb3enclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb3enclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB3ENCLRR)*/
pub struct MC_APB3ENCLRRrs;
impl crate::RegisterSpec for MC_APB3ENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_apb3enclrr::R`](R) reader structure
impl crate::Readable for MC_APB3ENCLRRrs {}
///`write(|w| ..)` method takes [`mc_apb3enclrr::W`](W) writer structure
impl crate::Writable for MC_APB3ENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_APB3ENCLRR to value 0
impl crate::Resettable for MC_APB3ENCLRRrs {}
