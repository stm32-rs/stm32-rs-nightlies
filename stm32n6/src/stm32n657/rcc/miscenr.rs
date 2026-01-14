///Register `MISCENR` reader
pub type R = crate::R<MISCENRrs>;
///Register `MISCENR` writer
pub type W = crate::W<MISCENRrs>;
///Field `DBGEN` reader - DBG enable
pub type DBGEN_R = crate::BitReader;
///Field `DBGEN` writer - DBG enable
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO1EN` reader - MCO1 enable
pub type MCO1EN_R = crate::BitReader;
///Field `MCO1EN` writer - MCO1 enable
pub type MCO1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO2EN` reader - MCO2 enable
pub type MCO2EN_R = crate::BitReader;
///Field `MCO2EN` writer - MCO2 enable
pub type MCO2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHYCOMPEN` reader - XSPIPHYCOMP enable
pub type XSPIPHYCOMPEN_R = crate::BitReader;
///Field `XSPIPHYCOMPEN` writer - XSPIPHYCOMP enable
pub type XSPIPHYCOMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEREN` reader - PER enable
pub type PEREN_R = crate::BitReader;
///Field `PEREN` writer - PER enable
pub type PEREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DBG enable
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCO1 enable
    #[inline(always)]
    pub fn mco1en(&self) -> MCO1EN_R {
        MCO1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MCO2 enable
    #[inline(always)]
    pub fn mco2en(&self) -> MCO2EN_R {
        MCO2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - XSPIPHYCOMP enable
    #[inline(always)]
    pub fn xspiphycompen(&self) -> XSPIPHYCOMPEN_R {
        XSPIPHYCOMPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - PER enable
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISCENR")
            .field("dbgen", &self.dbgen())
            .field("mco1en", &self.mco1en())
            .field("mco2en", &self.mco2en())
            .field("xspiphycompen", &self.xspiphycompen())
            .field("peren", &self.peren())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG enable
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<'_, MISCENRrs> {
        DBGEN_W::new(self, 0)
    }
    ///Bit 1 - MCO1 enable
    #[inline(always)]
    pub fn mco1en(&mut self) -> MCO1EN_W<'_, MISCENRrs> {
        MCO1EN_W::new(self, 1)
    }
    ///Bit 2 - MCO2 enable
    #[inline(always)]
    pub fn mco2en(&mut self) -> MCO2EN_W<'_, MISCENRrs> {
        MCO2EN_W::new(self, 2)
    }
    ///Bit 3 - XSPIPHYCOMP enable
    #[inline(always)]
    pub fn xspiphycompen(&mut self) -> XSPIPHYCOMPEN_W<'_, MISCENRrs> {
        XSPIPHYCOMPEN_W::new(self, 3)
    }
    ///Bit 6 - PER enable
    #[inline(always)]
    pub fn peren(&mut self) -> PEREN_W<'_, MISCENRrs> {
        PEREN_W::new(self, 6)
    }
}
/**RCC miscellaneous configuration enable register

You can [`read`](crate::Reg::read) this register and get [`miscenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MISCENR)*/
pub struct MISCENRrs;
impl crate::RegisterSpec for MISCENRrs {
    type Ux = u32;
}
///`read()` method returns [`miscenr::R`](R) reader structure
impl crate::Readable for MISCENRrs {}
///`write(|w| ..)` method takes [`miscenr::W`](W) writer structure
impl crate::Writable for MISCENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCENR to value 0
impl crate::Resettable for MISCENRrs {}
