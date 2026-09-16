///Register `MEMENR` reader
pub type R = crate::R<MEMENRrs>;
///Register `MEMENR` writer
pub type W = crate::W<MEMENRrs>;
///Field `AXISRAM3EN` reader - AXISRAM3 enable
pub type AXISRAM3EN_R = crate::BitReader;
///Field `AXISRAM3EN` writer - AXISRAM3 enable
pub type AXISRAM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4EN` reader - AXISRAM4 enable
pub type AXISRAM4EN_R = crate::BitReader;
///Field `AXISRAM4EN` writer - AXISRAM4 enable
pub type AXISRAM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5EN` reader - AXISRAM5 enable
pub type AXISRAM5EN_R = crate::BitReader;
///Field `AXISRAM5EN` writer - AXISRAM5 enable
pub type AXISRAM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6EN` reader - AXISRAM6 enable
pub type AXISRAM6EN_R = crate::BitReader;
///Field `AXISRAM6EN` writer - AXISRAM6 enable
pub type AXISRAM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1EN` reader - AHBSRAM1 enable
pub type AHBSRAM1EN_R = crate::BitReader;
///Field `AHBSRAM1EN` writer - AHBSRAM1 enable
pub type AHBSRAM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2EN` reader - AHBSRAM2 enable
pub type AHBSRAM2EN_R = crate::BitReader;
///Field `AHBSRAM2EN` writer - AHBSRAM2 enable
pub type AHBSRAM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMEN` reader - BKPSRAM enable
pub type BKPSRAMEN_R = crate::BitReader;
///Field `BKPSRAMEN` writer - BKPSRAM enable
pub type BKPSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1EN` reader - AXISRAM1 enable
pub type AXISRAM1EN_R = crate::BitReader;
///Field `AXISRAM1EN` writer - AXISRAM1 enable
pub type AXISRAM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2EN` reader - AXISRAM2 enable
pub type AXISRAM2EN_R = crate::BitReader;
///Field `AXISRAM2EN` writer - AXISRAM2 enable
pub type AXISRAM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMEN` reader - FLEXRAM enable
pub type FLEXRAMEN_R = crate::BitReader;
///Field `FLEXRAMEN` writer - FLEXRAM enable
pub type FLEXRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMEN` reader - NPUCACHERAM enable
pub type NPUCACHERAMEN_R = crate::BitReader;
///Field `NPUCACHERAMEN` writer - NPUCACHERAM enable
pub type NPUCACHERAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMEN` reader - VENCRAM enable
pub type VENCRAMEN_R = crate::BitReader;
///Field `VENCRAMEN` writer - VENCRAM enable
pub type VENCRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMEN` reader - BOOTROM enable
pub type BOOTROMEN_R = crate::BitReader;
///Field `BOOTROMEN` writer - BOOTROM enable
pub type BOOTROMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AXISRAM3 enable
    #[inline(always)]
    pub fn axisram3en(&self) -> AXISRAM3EN_R {
        AXISRAM3EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXISRAM4 enable
    #[inline(always)]
    pub fn axisram4en(&self) -> AXISRAM4EN_R {
        AXISRAM4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXISRAM5 enable
    #[inline(always)]
    pub fn axisram5en(&self) -> AXISRAM5EN_R {
        AXISRAM5EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXISRAM6 enable
    #[inline(always)]
    pub fn axisram6en(&self) -> AXISRAM6EN_R {
        AXISRAM6EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHBSRAM1 enable
    #[inline(always)]
    pub fn ahbsram1en(&self) -> AHBSRAM1EN_R {
        AHBSRAM1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHBSRAM2 enable
    #[inline(always)]
    pub fn ahbsram2en(&self) -> AHBSRAM2EN_R {
        AHBSRAM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BKPSRAM enable
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AXISRAM1 enable
    #[inline(always)]
    pub fn axisram1en(&self) -> AXISRAM1EN_R {
        AXISRAM1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXISRAM2 enable
    #[inline(always)]
    pub fn axisram2en(&self) -> AXISRAM2EN_R {
        AXISRAM2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FLEXRAM enable
    #[inline(always)]
    pub fn flexramen(&self) -> FLEXRAMEN_R {
        FLEXRAMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NPUCACHERAM enable
    #[inline(always)]
    pub fn npucacheramen(&self) -> NPUCACHERAMEN_R {
        NPUCACHERAMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - VENCRAM enable
    #[inline(always)]
    pub fn vencramen(&self) -> VENCRAMEN_R {
        VENCRAMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BOOTROM enable
    #[inline(always)]
    pub fn bootromen(&self) -> BOOTROMEN_R {
        BOOTROMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMENR")
            .field("axisram3en", &self.axisram3en())
            .field("axisram4en", &self.axisram4en())
            .field("axisram5en", &self.axisram5en())
            .field("axisram6en", &self.axisram6en())
            .field("ahbsram1en", &self.ahbsram1en())
            .field("ahbsram2en", &self.ahbsram2en())
            .field("bkpsramen", &self.bkpsramen())
            .field("axisram1en", &self.axisram1en())
            .field("axisram2en", &self.axisram2en())
            .field("flexramen", &self.flexramen())
            .field("npucacheramen", &self.npucacheramen())
            .field("vencramen", &self.vencramen())
            .field("bootromen", &self.bootromen())
            .finish()
    }
}
impl W {
    ///Bit 0 - AXISRAM3 enable
    #[inline(always)]
    pub fn axisram3en(&mut self) -> AXISRAM3EN_W<'_, MEMENRrs> {
        AXISRAM3EN_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 enable
    #[inline(always)]
    pub fn axisram4en(&mut self) -> AXISRAM4EN_W<'_, MEMENRrs> {
        AXISRAM4EN_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 enable
    #[inline(always)]
    pub fn axisram5en(&mut self) -> AXISRAM5EN_W<'_, MEMENRrs> {
        AXISRAM5EN_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 enable
    #[inline(always)]
    pub fn axisram6en(&mut self) -> AXISRAM6EN_W<'_, MEMENRrs> {
        AXISRAM6EN_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 enable
    #[inline(always)]
    pub fn ahbsram1en(&mut self) -> AHBSRAM1EN_W<'_, MEMENRrs> {
        AHBSRAM1EN_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 enable
    #[inline(always)]
    pub fn ahbsram2en(&mut self) -> AHBSRAM2EN_W<'_, MEMENRrs> {
        AHBSRAM2EN_W::new(self, 5)
    }
    ///Bit 6 - BKPSRAM enable
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<'_, MEMENRrs> {
        BKPSRAMEN_W::new(self, 6)
    }
    ///Bit 7 - AXISRAM1 enable
    #[inline(always)]
    pub fn axisram1en(&mut self) -> AXISRAM1EN_W<'_, MEMENRrs> {
        AXISRAM1EN_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 enable
    #[inline(always)]
    pub fn axisram2en(&mut self) -> AXISRAM2EN_W<'_, MEMENRrs> {
        AXISRAM2EN_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM enable
    #[inline(always)]
    pub fn flexramen(&mut self) -> FLEXRAMEN_W<'_, MEMENRrs> {
        FLEXRAMEN_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM enable
    #[inline(always)]
    pub fn npucacheramen(&mut self) -> NPUCACHERAMEN_W<'_, MEMENRrs> {
        NPUCACHERAMEN_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM enable
    #[inline(always)]
    pub fn vencramen(&mut self) -> VENCRAMEN_W<'_, MEMENRrs> {
        VENCRAMEN_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM enable
    #[inline(always)]
    pub fn bootromen(&mut self) -> BOOTROMEN_W<'_, MEMENRrs> {
        BOOTROMEN_W::new(self, 12)
    }
}
/**RCC memory enable register

You can [`read`](crate::Reg::read) this register and get [`memenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMENR)*/
pub struct MEMENRrs;
impl crate::RegisterSpec for MEMENRrs {
    type Ux = u32;
}
///`read()` method returns [`memenr::R`](R) reader structure
impl crate::Readable for MEMENRrs {}
///`write(|w| ..)` method takes [`memenr::W`](W) writer structure
impl crate::Writable for MEMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMENR to value 0x13ff
impl crate::Resettable for MEMENRrs {
    const RESET_VALUE: u32 = 0x13ff;
}
