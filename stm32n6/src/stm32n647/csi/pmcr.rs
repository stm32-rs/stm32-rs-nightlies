///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
///Field `FRXMDL0` reader - Force to Rx mode the data lane 0
pub type FRXMDL0_R = crate::BitReader;
///Field `FRXMDL0` writer - Force to Rx mode the data lane 0
pub type FRXMDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRXMDL1` reader - Force to Rx mode the data lane 1
pub type FRXMDL1_R = crate::BitReader;
///Field `FRXMDL1` writer - Force to Rx mode the data lane 1
pub type FRXMDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMDL0` reader - Force to Tx Stop mode the data lane 0
pub type FTXSMDL0_R = crate::BitReader;
///Field `FTXSMDL0` writer - Force to Tx Stop mode the data lane 0
pub type FTXSMDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDL` reader - Disable turn-around data lane 0
pub type DTDL_R = crate::BitReader;
///Field `DTDL` writer - Disable turn-around data lane 0
pub type DTDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTDL0` reader - Turn-around request data lane 0
pub type RTDL0_R = crate::BitReader;
///Field `RTDL0` writer - Turn-around request data lane 0
pub type RTDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TUESDL0` reader - Tx ULP escape-mode data lane 0
pub type TUESDL0_R = crate::BitReader;
///Field `TUESDL0` writer - Tx ULP escape-mode data lane 0
pub type TUESDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TUEXDL0` reader - Tx ULP exit sequence data lane 0
pub type TUEXDL0_R = crate::BitReader;
///Field `TUEXDL0` writer - Tx ULP exit sequence data lane 0
pub type TUEXDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Force to Rx mode the data lane 0
    #[inline(always)]
    pub fn frxmdl0(&self) -> FRXMDL0_R {
        FRXMDL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Force to Rx mode the data lane 1
    #[inline(always)]
    pub fn frxmdl1(&self) -> FRXMDL1_R {
        FRXMDL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Force to Tx Stop mode the data lane 0
    #[inline(always)]
    pub fn ftxsmdl0(&self) -> FTXSMDL0_R {
        FTXSMDL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Disable turn-around data lane 0
    #[inline(always)]
    pub fn dtdl(&self) -> DTDL_R {
        DTDL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Turn-around request data lane 0
    #[inline(always)]
    pub fn rtdl0(&self) -> RTDL0_R {
        RTDL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Tx ULP escape-mode data lane 0
    #[inline(always)]
    pub fn tuesdl0(&self) -> TUESDL0_R {
        TUESDL0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Tx ULP exit sequence data lane 0
    #[inline(always)]
    pub fn tuexdl0(&self) -> TUEXDL0_R {
        TUEXDL0_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("frxmdl0", &self.frxmdl0())
            .field("frxmdl1", &self.frxmdl1())
            .field("ftxsmdl0", &self.ftxsmdl0())
            .field("dtdl", &self.dtdl())
            .field("rtdl0", &self.rtdl0())
            .field("tuesdl0", &self.tuesdl0())
            .field("tuexdl0", &self.tuexdl0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Force to Rx mode the data lane 0
    #[inline(always)]
    pub fn frxmdl0(&mut self) -> FRXMDL0_W<'_, PMCRrs> {
        FRXMDL0_W::new(self, 0)
    }
    ///Bit 1 - Force to Rx mode the data lane 1
    #[inline(always)]
    pub fn frxmdl1(&mut self) -> FRXMDL1_W<'_, PMCRrs> {
        FRXMDL1_W::new(self, 1)
    }
    ///Bit 2 - Force to Tx Stop mode the data lane 0
    #[inline(always)]
    pub fn ftxsmdl0(&mut self) -> FTXSMDL0_W<'_, PMCRrs> {
        FTXSMDL0_W::new(self, 2)
    }
    ///Bit 4 - Disable turn-around data lane 0
    #[inline(always)]
    pub fn dtdl(&mut self) -> DTDL_W<'_, PMCRrs> {
        DTDL_W::new(self, 4)
    }
    ///Bit 8 - Turn-around request data lane 0
    #[inline(always)]
    pub fn rtdl0(&mut self) -> RTDL0_W<'_, PMCRrs> {
        RTDL0_W::new(self, 8)
    }
    ///Bit 12 - Tx ULP escape-mode data lane 0
    #[inline(always)]
    pub fn tuesdl0(&mut self) -> TUESDL0_W<'_, PMCRrs> {
        TUESDL0_W::new(self, 12)
    }
    ///Bit 16 - Tx ULP exit sequence data lane 0
    #[inline(always)]
    pub fn tuexdl0(&mut self) -> TUEXDL0_W<'_, PMCRrs> {
        TUEXDL0_W::new(self, 16)
    }
}
/**CSI PHY mode control register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:PMCR)*/
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcr::R`](R) reader structure
impl crate::Readable for PMCRrs {}
///`write(|w| ..)` method takes [`pmcr::W`](W) writer structure
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR to value 0x0001_1003
impl crate::Resettable for PMCRrs {
    const RESET_VALUE: u32 = 0x0001_1003;
}
