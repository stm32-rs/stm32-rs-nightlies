#[doc = "Register `DDRPHYC_DDR3_MR0` reader"]
pub type R = crate::R<DDRPHYC_DDR3_MR0rs>;
#[doc = "Register `DDRPHYC_DDR3_MR0` writer"]
pub type W = crate::W<DDRPHYC_DDR3_MR0rs>;
#[doc = "Field `BL` reader - BL"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - BL"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CL0` reader - CL0"]
pub type CL0_R = crate::BitReader;
#[doc = "Field `CL0` writer - CL0"]
pub type CL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT` reader - BT"]
pub type BT_R = crate::BitReader;
#[doc = "Field `BT` writer - BT"]
pub type BT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL` reader - CL"]
pub type CL_R = crate::FieldReader;
#[doc = "Field `CL` writer - CL"]
pub type CL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TM` reader - TM"]
pub type TM_R = crate::BitReader;
#[doc = "Field `TM` writer - TM"]
pub type TM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - DR"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - DR"]
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - WR"]
pub type WR_R = crate::FieldReader;
#[doc = "Field `WR` writer - WR"]
pub type WR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD` reader - PD"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - PD"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RSVD_R = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RSVD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CL0"]
    #[inline(always)]
    pub fn cl0(&self) -> CL0_R {
        CL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    pub fn bt(&self) -> BT_R {
        BT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - CL"]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - WR"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BL"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<DDRPHYC_DDR3_MR0rs> {
        BL_W::new(self, 0)
    }
    #[doc = "Bit 2 - CL0"]
    #[inline(always)]
    #[must_use]
    pub fn cl0(&mut self) -> CL0_W<DDRPHYC_DDR3_MR0rs> {
        CL0_W::new(self, 2)
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    #[must_use]
    pub fn bt(&mut self) -> BT_W<DDRPHYC_DDR3_MR0rs> {
        BT_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - CL"]
    #[inline(always)]
    #[must_use]
    pub fn cl(&mut self) -> CL_W<DDRPHYC_DDR3_MR0rs> {
        CL_W::new(self, 4)
    }
    #[doc = "Bit 7 - TM"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<DDRPHYC_DDR3_MR0rs> {
        TM_W::new(self, 7)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<DDRPHYC_DDR3_MR0rs> {
        DR_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - WR"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<DDRPHYC_DDR3_MR0rs> {
        WR_W::new(self, 9)
    }
    #[doc = "Bit 12 - PD"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<DDRPHYC_DDR3_MR0rs> {
        PD_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - RSVD"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RSVD_W<DDRPHYC_DDR3_MR0rs> {
        RSVD_W::new(self, 13)
    }
}
#[doc = "DDRPHYC MR0 register for DDR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ddr3_mr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DDR3_MR0rs;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR0rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ddrphyc_ddr3_mr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ddr3_mr0::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR0 to value 0x0a52"]
impl crate::Resettable for DDRPHYC_DDR3_MR0rs {
    const RESET_VALUE: u16 = 0x0a52;
}
