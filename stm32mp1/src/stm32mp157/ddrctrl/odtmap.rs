///Register `ODTMAP` reader
pub type R = crate::R<ODTMAPrs>;
///Register `ODTMAP` writer
pub type W = crate::W<ODTMAPrs>;
///Field `RANK0_WR_ODT` reader - RANK0_WR_ODT
pub type RANK0_WR_ODT_R = crate::BitReader;
///Field `RANK0_WR_ODT` writer - RANK0_WR_ODT
pub type RANK0_WR_ODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RANK0_RD_ODT` reader - RANK0_RD_ODT
pub type RANK0_RD_ODT_R = crate::BitReader;
///Field `RANK0_RD_ODT` writer - RANK0_RD_ODT
pub type RANK0_RD_ODT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RANK0_WR_ODT
    #[inline(always)]
    pub fn rank0_wr_odt(&self) -> RANK0_WR_ODT_R {
        RANK0_WR_ODT_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - RANK0_RD_ODT
    #[inline(always)]
    pub fn rank0_rd_odt(&self) -> RANK0_RD_ODT_R {
        RANK0_RD_ODT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODTMAP")
            .field("rank0_wr_odt", &self.rank0_wr_odt())
            .field("rank0_rd_odt", &self.rank0_rd_odt())
            .finish()
    }
}
impl W {
    ///Bit 0 - RANK0_WR_ODT
    #[inline(always)]
    pub fn rank0_wr_odt(&mut self) -> RANK0_WR_ODT_W<'_, ODTMAPrs> {
        RANK0_WR_ODT_W::new(self, 0)
    }
    ///Bit 4 - RANK0_RD_ODT
    #[inline(always)]
    pub fn rank0_rd_odt(&mut self) -> RANK0_RD_ODT_W<'_, ODTMAPrs> {
        RANK0_RD_ODT_W::new(self, 4)
    }
}
/**DDRCTRL ODT/Rank map register

You can [`read`](crate::Reg::read) this register and get [`odtmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odtmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ODTMAP)*/
pub struct ODTMAPrs;
impl crate::RegisterSpec for ODTMAPrs {
    type Ux = u32;
}
///`read()` method returns [`odtmap::R`](R) reader structure
impl crate::Readable for ODTMAPrs {}
///`write(|w| ..)` method takes [`odtmap::W`](W) writer structure
impl crate::Writable for ODTMAPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODTMAP to value 0x11
impl crate::Resettable for ODTMAPrs {
    const RESET_VALUE: u32 = 0x11;
}
