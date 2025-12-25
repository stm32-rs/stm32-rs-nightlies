///Register `DDR3_MR0` reader
pub type R = crate::R<DDR3_MR0rs>;
///Register `DDR3_MR0` writer
pub type W = crate::W<DDR3_MR0rs>;
///Field `BL` reader - BL
pub type BL_R = crate::FieldReader;
///Field `BL` writer - BL
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CL0` reader - CL0
pub type CL0_R = crate::BitReader;
///Field `CL0` writer - CL0
pub type CL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT` reader - BT
pub type BT_R = crate::BitReader;
///Field `BT` writer - BT
pub type BT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CL` reader - CL
pub type CL_R = crate::FieldReader;
///Field `CL` writer - CL
pub type CL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TM` reader - TM
pub type TM_R = crate::BitReader;
///Field `TM` writer - TM
pub type TM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DR` reader - DR
pub type DR_R = crate::BitReader;
///Field `DR` writer - DR
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR` reader - WR
pub type WR_R = crate::FieldReader;
///Field `WR` writer - WR
pub type WR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PD` reader - PD
pub type PD_R = crate::BitReader;
///Field `PD` writer - PD
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader - RSVD
pub type RSVD_R = crate::FieldReader;
///Field `RSVD` writer - RSVD
pub type RSVD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - BL
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - CL0
    #[inline(always)]
    pub fn cl0(&self) -> CL0_R {
        CL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BT
    #[inline(always)]
    pub fn bt(&self) -> BT_R {
        BT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - CL
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TM
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - WR
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12 - PD
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - RSVD
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR3_MR0")
            .field("bl", &self.bl())
            .field("cl0", &self.cl0())
            .field("bt", &self.bt())
            .field("cl", &self.cl())
            .field("tm", &self.tm())
            .field("dr", &self.dr())
            .field("wr", &self.wr())
            .field("pd", &self.pd())
            .field("rsvd", &self.rsvd())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - BL
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, DDR3_MR0rs> {
        BL_W::new(self, 0)
    }
    ///Bit 2 - CL0
    #[inline(always)]
    pub fn cl0(&mut self) -> CL0_W<'_, DDR3_MR0rs> {
        CL0_W::new(self, 2)
    }
    ///Bit 3 - BT
    #[inline(always)]
    pub fn bt(&mut self) -> BT_W<'_, DDR3_MR0rs> {
        BT_W::new(self, 3)
    }
    ///Bits 4:6 - CL
    #[inline(always)]
    pub fn cl(&mut self) -> CL_W<'_, DDR3_MR0rs> {
        CL_W::new(self, 4)
    }
    ///Bit 7 - TM
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W<'_, DDR3_MR0rs> {
        TM_W::new(self, 7)
    }
    ///Bit 8 - DR
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, DDR3_MR0rs> {
        DR_W::new(self, 8)
    }
    ///Bits 9:11 - WR
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W<'_, DDR3_MR0rs> {
        WR_W::new(self, 9)
    }
    ///Bit 12 - PD
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<'_, DDR3_MR0rs> {
        PD_W::new(self, 12)
    }
    ///Bits 13:15 - RSVD
    #[inline(always)]
    pub fn rsvd(&mut self) -> RSVD_W<'_, DDR3_MR0rs> {
        RSVD_W::new(self, 13)
    }
}
/**DDRPHYC MR0 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR0)*/
pub struct DDR3_MR0rs;
impl crate::RegisterSpec for DDR3_MR0rs {
    type Ux = u16;
}
///`read()` method returns [`ddr3_mr0::R`](R) reader structure
impl crate::Readable for DDR3_MR0rs {}
///`write(|w| ..)` method takes [`ddr3_mr0::W`](W) writer structure
impl crate::Writable for DDR3_MR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DDR3_MR0 to value 0x0a52
impl crate::Resettable for DDR3_MR0rs {
    const RESET_VALUE: u16 = 0x0a52;
}
