///Register `MSTR` reader
pub type R = crate::R<MSTRrs>;
///Register `MSTR` writer
pub type W = crate::W<MSTRrs>;
///Field `DDR3` reader - DDR3
pub type DDR3_R = crate::BitReader;
///Field `DDR3` writer - DDR3
pub type DDR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDDR2` reader - LPDDR2
pub type LPDDR2_R = crate::BitReader;
///Field `LPDDR2` writer - LPDDR2
pub type LPDDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDDR3` reader - LPDDR3
pub type LPDDR3_R = crate::BitReader;
///Field `LPDDR3` writer - LPDDR3
pub type LPDDR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BURSTCHOP` reader - BURSTCHOP
pub type BURSTCHOP_R = crate::BitReader;
///Field `BURSTCHOP` writer - BURSTCHOP
pub type BURSTCHOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_2T_TIMING_MODE` reader - EN_2T_TIMING_MODE
pub type EN_2T_TIMING_MODE_R = crate::BitReader;
///Field `EN_2T_TIMING_MODE` writer - EN_2T_TIMING_MODE
pub type EN_2T_TIMING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_BUS_WIDTH` reader - DATA_BUS_WIDTH
pub type DATA_BUS_WIDTH_R = crate::FieldReader;
///Field `DATA_BUS_WIDTH` writer - DATA_BUS_WIDTH
pub type DATA_BUS_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DLL_OFF_MODE` reader - DLL_OFF_MODE
pub type DLL_OFF_MODE_R = crate::BitReader;
///Field `DLL_OFF_MODE` writer - DLL_OFF_MODE
pub type DLL_OFF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BURST_RDWR` reader - BURST_RDWR
pub type BURST_RDWR_R = crate::FieldReader;
///Field `BURST_RDWR` writer - BURST_RDWR
pub type BURST_RDWR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - DDR3
    #[inline(always)]
    pub fn ddr3(&self) -> DDR3_R {
        DDR3_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - LPDDR2
    #[inline(always)]
    pub fn lpddr2(&self) -> LPDDR2_R {
        LPDDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPDDR3
    #[inline(always)]
    pub fn lpddr3(&self) -> LPDDR3_R {
        LPDDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - BURSTCHOP
    #[inline(always)]
    pub fn burstchop(&self) -> BURSTCHOP_R {
        BURSTCHOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - EN_2T_TIMING_MODE
    #[inline(always)]
    pub fn en_2t_timing_mode(&self) -> EN_2T_TIMING_MODE_R {
        EN_2T_TIMING_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 12:13 - DATA_BUS_WIDTH
    #[inline(always)]
    pub fn data_bus_width(&self) -> DATA_BUS_WIDTH_R {
        DATA_BUS_WIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - DLL_OFF_MODE
    #[inline(always)]
    pub fn dll_off_mode(&self) -> DLL_OFF_MODE_R {
        DLL_OFF_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - BURST_RDWR
    #[inline(always)]
    pub fn burst_rdwr(&self) -> BURST_RDWR_R {
        BURST_RDWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTR")
            .field("ddr3", &self.ddr3())
            .field("lpddr2", &self.lpddr2())
            .field("lpddr3", &self.lpddr3())
            .field("burstchop", &self.burstchop())
            .field("en_2t_timing_mode", &self.en_2t_timing_mode())
            .field("data_bus_width", &self.data_bus_width())
            .field("dll_off_mode", &self.dll_off_mode())
            .field("burst_rdwr", &self.burst_rdwr())
            .finish()
    }
}
impl W {
    ///Bit 0 - DDR3
    #[inline(always)]
    pub fn ddr3(&mut self) -> DDR3_W<'_, MSTRrs> {
        DDR3_W::new(self, 0)
    }
    ///Bit 2 - LPDDR2
    #[inline(always)]
    pub fn lpddr2(&mut self) -> LPDDR2_W<'_, MSTRrs> {
        LPDDR2_W::new(self, 2)
    }
    ///Bit 3 - LPDDR3
    #[inline(always)]
    pub fn lpddr3(&mut self) -> LPDDR3_W<'_, MSTRrs> {
        LPDDR3_W::new(self, 3)
    }
    ///Bit 9 - BURSTCHOP
    #[inline(always)]
    pub fn burstchop(&mut self) -> BURSTCHOP_W<'_, MSTRrs> {
        BURSTCHOP_W::new(self, 9)
    }
    ///Bit 10 - EN_2T_TIMING_MODE
    #[inline(always)]
    pub fn en_2t_timing_mode(&mut self) -> EN_2T_TIMING_MODE_W<'_, MSTRrs> {
        EN_2T_TIMING_MODE_W::new(self, 10)
    }
    ///Bits 12:13 - DATA_BUS_WIDTH
    #[inline(always)]
    pub fn data_bus_width(&mut self) -> DATA_BUS_WIDTH_W<'_, MSTRrs> {
        DATA_BUS_WIDTH_W::new(self, 12)
    }
    ///Bit 15 - DLL_OFF_MODE
    #[inline(always)]
    pub fn dll_off_mode(&mut self) -> DLL_OFF_MODE_W<'_, MSTRrs> {
        DLL_OFF_MODE_W::new(self, 15)
    }
    ///Bits 16:19 - BURST_RDWR
    #[inline(always)]
    pub fn burst_rdwr(&mut self) -> BURST_RDWR_W<'_, MSTRrs> {
        BURST_RDWR_W::new(self, 16)
    }
}
/**DDRCTRL master register 0

You can [`read`](crate::Reg::read) this register and get [`mstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MSTR)*/
pub struct MSTRrs;
impl crate::RegisterSpec for MSTRrs {
    type Ux = u32;
}
///`read()` method returns [`mstr::R`](R) reader structure
impl crate::Readable for MSTRrs {}
///`write(|w| ..)` method takes [`mstr::W`](W) writer structure
impl crate::Writable for MSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTR to value 0x0004_0001
impl crate::Resettable for MSTRrs {
    const RESET_VALUE: u32 = 0x0004_0001;
}
