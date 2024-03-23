#[doc = "Register `DDRCTRL_MSTR` reader"]
pub type R = crate::R<DDRCTRL_MSTRrs>;
#[doc = "Register `DDRCTRL_MSTR` writer"]
pub type W = crate::W<DDRCTRL_MSTRrs>;
#[doc = "Field `DDR3` reader - DDR3"]
pub type DDR3_R = crate::BitReader;
#[doc = "Field `DDR3` writer - DDR3"]
pub type DDR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPDDR2` reader - LPDDR2"]
pub type LPDDR2_R = crate::BitReader;
#[doc = "Field `LPDDR2` writer - LPDDR2"]
pub type LPDDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPDDR3` reader - LPDDR3"]
pub type LPDDR3_R = crate::BitReader;
#[doc = "Field `LPDDR3` writer - LPDDR3"]
pub type LPDDR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURSTCHOP` reader - BURSTCHOP"]
pub type BURSTCHOP_R = crate::BitReader;
#[doc = "Field `BURSTCHOP` writer - BURSTCHOP"]
pub type BURSTCHOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_2T_TIMING_MODE` reader - EN_2T_TIMING_MODE"]
pub type EN_2T_TIMING_MODE_R = crate::BitReader;
#[doc = "Field `EN_2T_TIMING_MODE` writer - EN_2T_TIMING_MODE"]
pub type EN_2T_TIMING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_BUS_WIDTH` reader - DATA_BUS_WIDTH"]
pub type DATA_BUS_WIDTH_R = crate::FieldReader;
#[doc = "Field `DATA_BUS_WIDTH` writer - DATA_BUS_WIDTH"]
pub type DATA_BUS_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DLL_OFF_MODE` reader - DLL_OFF_MODE"]
pub type DLL_OFF_MODE_R = crate::BitReader;
#[doc = "Field `DLL_OFF_MODE` writer - DLL_OFF_MODE"]
pub type DLL_OFF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_RDWR` reader - BURST_RDWR"]
pub type BURST_RDWR_R = crate::FieldReader;
#[doc = "Field `BURST_RDWR` writer - BURST_RDWR"]
pub type BURST_RDWR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    pub fn ddr3(&self) -> DDR3_R {
        DDR3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    pub fn lpddr2(&self) -> LPDDR2_R {
        LPDDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    pub fn lpddr3(&self) -> LPDDR3_R {
        LPDDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    pub fn burstchop(&self) -> BURSTCHOP_R {
        BURSTCHOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    pub fn en_2t_timing_mode(&self) -> EN_2T_TIMING_MODE_R {
        EN_2T_TIMING_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    pub fn data_bus_width(&self) -> DATA_BUS_WIDTH_R {
        DATA_BUS_WIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    pub fn dll_off_mode(&self) -> DLL_OFF_MODE_R {
        DLL_OFF_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    pub fn burst_rdwr(&self) -> BURST_RDWR_R {
        BURST_RDWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    #[must_use]
    pub fn ddr3(&mut self) -> DDR3_W<DDRCTRL_MSTRrs> {
        DDR3_W::new(self, 0)
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr2(&mut self) -> LPDDR2_W<DDRCTRL_MSTRrs> {
        LPDDR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr3(&mut self) -> LPDDR3_W<DDRCTRL_MSTRrs> {
        LPDDR3_W::new(self, 3)
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    #[must_use]
    pub fn burstchop(&mut self) -> BURSTCHOP_W<DDRCTRL_MSTRrs> {
        BURSTCHOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn en_2t_timing_mode(&mut self) -> EN_2T_TIMING_MODE_W<DDRCTRL_MSTRrs> {
        EN_2T_TIMING_MODE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn data_bus_width(&mut self) -> DATA_BUS_WIDTH_W<DDRCTRL_MSTRrs> {
        DATA_BUS_WIDTH_W::new(self, 12)
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn dll_off_mode(&mut self) -> DLL_OFF_MODE_W<DDRCTRL_MSTRrs> {
        DLL_OFF_MODE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    #[must_use]
    pub fn burst_rdwr(&mut self) -> BURST_RDWR_W<DDRCTRL_MSTRrs> {
        BURST_RDWR_W::new(self, 16)
    }
}
#[doc = "DDRCTRL master register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_mstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_MSTRrs;
impl crate::RegisterSpec for DDRCTRL_MSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_mstr::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_MSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_mstr::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_MSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_MSTR to value 0x0004_0001"]
impl crate::Resettable for DDRCTRL_MSTRrs {
    const RESET_VALUE: u32 = 0x0004_0001;
}
