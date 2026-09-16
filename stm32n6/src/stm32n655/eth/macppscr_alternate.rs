///Register `MACPPSCR_alternate` reader
pub type R = crate::R<MACPPSCR_ALTERNATErs>;
///Register `MACPPSCR_alternate` writer
pub type W = crate::W<MACPPSCR_ALTERNATErs>;
///Field `PPSCMD` reader - Flexible PPS Output 0 (eth_ptp_pps_out) Control
pub type PPSCMD_R = crate::FieldReader;
///Field `PPSCMD` writer - Flexible PPS Output 0 (eth_ptp_pps_out) Control
pub type PPSCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPSEN0` reader - Flexible PPS Output 0 Mode Enable
pub type PPSEN0_R = crate::BitReader;
///Field `PPSEN0` writer - Flexible PPS Output 0 Mode Enable
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output 0
pub type TRGTMODSEL0_R = crate::FieldReader;
///Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output 0
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCGREN0` reader - MCGR Mode Enable for PPS Output 0
pub type MCGREN0_R = crate::BitReader;
///Field `MCGREN0` writer - MCGR Mode Enable for PPS Output 0
pub type MCGREN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PPSCMD1` reader - Flexible PPS Output 1 Control
pub type PPSCMD1_R = crate::FieldReader;
///Field `PPSCMD1` writer - Flexible PPS Output 1 Control
pub type PPSCMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRGTMODSEL1` reader - Target Time Register Mode for PPS Output 1
pub type TRGTMODSEL1_R = crate::FieldReader;
///Field `TRGTMODSEL1` writer - Target Time Register Mode for PPS Output 1
pub type TRGTMODSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCGREN1` reader - MCGR Mode Enable for PPS Output 1
pub type MCGREN1_R = crate::BitReader;
///Field `MCGREN1` writer - MCGR Mode Enable for PPS Output 1
pub type MCGREN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESEL` reader - Time Select
pub type TIMESEL_R = crate::BitReader;
///Field `TIMESEL` writer - Time Select
pub type TIMESEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Flexible PPS Output 0 (eth_ptp_pps_out) Control
    #[inline(always)]
    pub fn ppscmd(&self) -> PPSCMD_R {
        PPSCMD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Flexible PPS Output 0 Mode Enable
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output 0
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - MCGR Mode Enable for PPS Output 0
    #[inline(always)]
    pub fn mcgren0(&self) -> MCGREN0_R {
        MCGREN0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Flexible PPS Output 1 Control
    #[inline(always)]
    pub fn ppscmd1(&self) -> PPSCMD1_R {
        PPSCMD1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 13:14 - Target Time Register Mode for PPS Output 1
    #[inline(always)]
    pub fn trgtmodsel1(&self) -> TRGTMODSEL1_R {
        TRGTMODSEL1_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - MCGR Mode Enable for PPS Output 1
    #[inline(always)]
    pub fn mcgren1(&self) -> MCGREN1_R {
        MCGREN1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 28 - Time Select
    #[inline(always)]
    pub fn timesel(&self) -> TIMESEL_R {
        TIMESEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSCR_alternate")
            .field("ppscmd", &self.ppscmd())
            .field("ppsen0", &self.ppsen0())
            .field("trgtmodsel0", &self.trgtmodsel0())
            .field("mcgren0", &self.mcgren0())
            .field("ppscmd1", &self.ppscmd1())
            .field("trgtmodsel1", &self.trgtmodsel1())
            .field("mcgren1", &self.mcgren1())
            .field("timesel", &self.timesel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Flexible PPS Output 0 (eth_ptp_pps_out) Control
    #[inline(always)]
    pub fn ppscmd(&mut self) -> PPSCMD_W<'_, MACPPSCR_ALTERNATErs> {
        PPSCMD_W::new(self, 0)
    }
    ///Bit 4 - Flexible PPS Output 0 Mode Enable
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W<'_, MACPPSCR_ALTERNATErs> {
        PPSEN0_W::new(self, 4)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output 0
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<'_, MACPPSCR_ALTERNATErs> {
        TRGTMODSEL0_W::new(self, 5)
    }
    ///Bit 7 - MCGR Mode Enable for PPS Output 0
    #[inline(always)]
    pub fn mcgren0(&mut self) -> MCGREN0_W<'_, MACPPSCR_ALTERNATErs> {
        MCGREN0_W::new(self, 7)
    }
    ///Bits 8:11 - Flexible PPS Output 1 Control
    #[inline(always)]
    pub fn ppscmd1(&mut self) -> PPSCMD1_W<'_, MACPPSCR_ALTERNATErs> {
        PPSCMD1_W::new(self, 8)
    }
    ///Bits 13:14 - Target Time Register Mode for PPS Output 1
    #[inline(always)]
    pub fn trgtmodsel1(&mut self) -> TRGTMODSEL1_W<'_, MACPPSCR_ALTERNATErs> {
        TRGTMODSEL1_W::new(self, 13)
    }
    ///Bit 15 - MCGR Mode Enable for PPS Output 1
    #[inline(always)]
    pub fn mcgren1(&mut self) -> MCGREN1_W<'_, MACPPSCR_ALTERNATErs> {
        MCGREN1_W::new(self, 15)
    }
    ///Bit 28 - Time Select
    #[inline(always)]
    pub fn timesel(&mut self) -> TIMESEL_W<'_, MACPPSCR_ALTERNATErs> {
        TIMESEL_W::new(self, 28)
    }
}
/**PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSCR_alternate)*/
pub struct MACPPSCR_ALTERNATErs;
impl crate::RegisterSpec for MACPPSCR_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`macppscr_alternate::R`](R) reader structure
impl crate::Readable for MACPPSCR_ALTERNATErs {}
///`write(|w| ..)` method takes [`macppscr_alternate::W`](W) writer structure
impl crate::Writable for MACPPSCR_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSCR_alternate to value 0
impl crate::Resettable for MACPPSCR_ALTERNATErs {}
