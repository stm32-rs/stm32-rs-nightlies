///Register `MTLESTCR` reader
pub type R = crate::R<MTLESTCRrs>;
///Register `MTLESTCR` writer
pub type W = crate::W<MTLESTCRrs>;
///Field `EEST` reader - Enable EST
pub type EEST_R = crate::BitReader;
///Field `EEST` writer - Enable EST
pub type EEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSWL` reader - Switch to S/W owned list
pub type SSWL_R = crate::BitReader;
///Field `SSWL` writer - Switch to S/W owned list
pub type SSWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDBF` reader - Do not Drop frames during Frame Size Error
pub type DDBF_R = crate::BitReader;
///Field `DDBF` writer - Do not Drop frames during Frame Size Error
pub type DDBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFBS` reader - Drop Frames causing Scheduling Error
pub type DFBS_R = crate::BitReader;
///Field `DFBS` writer - Drop Frames causing Scheduling Error
pub type DFBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCSE` reader - Loop Count to report Scheduling Error
pub type LCSE_R = crate::FieldReader;
///Field `LCSE` writer - Loop Count to report Scheduling Error
pub type LCSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TILS` reader - Time Interval Left Shift Amount
pub type TILS_R = crate::FieldReader;
///Field `TILS` writer - Time Interval Left Shift Amount
pub type TILS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CTOV` reader - Current Time Offset Value
pub type CTOV_R = crate::FieldReader<u16>;
///Field `CTOV` writer - Current Time Offset Value
pub type CTOV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PTOV` reader - PTP Time Offset Value
pub type PTOV_R = crate::FieldReader;
///Field `PTOV` writer - PTP Time Offset Value
pub type PTOV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Enable EST
    #[inline(always)]
    pub fn eest(&self) -> EEST_R {
        EEST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Switch to S/W owned list
    #[inline(always)]
    pub fn sswl(&self) -> SSWL_R {
        SSWL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Do not Drop frames during Frame Size Error
    #[inline(always)]
    pub fn ddbf(&self) -> DDBF_R {
        DDBF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Drop Frames causing Scheduling Error
    #[inline(always)]
    pub fn dfbs(&self) -> DFBS_R {
        DFBS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Loop Count to report Scheduling Error
    #[inline(always)]
    pub fn lcse(&self) -> LCSE_R {
        LCSE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - Time Interval Left Shift Amount
    #[inline(always)]
    pub fn tils(&self) -> TILS_R {
        TILS_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:23 - Current Time Offset Value
    #[inline(always)]
    pub fn ctov(&self) -> CTOV_R {
        CTOV_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    ///Bits 24:31 - PTP Time Offset Value
    #[inline(always)]
    pub fn ptov(&self) -> PTOV_R {
        PTOV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTCR")
            .field("eest", &self.eest())
            .field("sswl", &self.sswl())
            .field("ddbf", &self.ddbf())
            .field("dfbs", &self.dfbs())
            .field("lcse", &self.lcse())
            .field("tils", &self.tils())
            .field("ctov", &self.ctov())
            .field("ptov", &self.ptov())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable EST
    #[inline(always)]
    pub fn eest(&mut self) -> EEST_W<'_, MTLESTCRrs> {
        EEST_W::new(self, 0)
    }
    ///Bit 1 - Switch to S/W owned list
    #[inline(always)]
    pub fn sswl(&mut self) -> SSWL_W<'_, MTLESTCRrs> {
        SSWL_W::new(self, 1)
    }
    ///Bit 4 - Do not Drop frames during Frame Size Error
    #[inline(always)]
    pub fn ddbf(&mut self) -> DDBF_W<'_, MTLESTCRrs> {
        DDBF_W::new(self, 4)
    }
    ///Bit 5 - Drop Frames causing Scheduling Error
    #[inline(always)]
    pub fn dfbs(&mut self) -> DFBS_W<'_, MTLESTCRrs> {
        DFBS_W::new(self, 5)
    }
    ///Bits 6:7 - Loop Count to report Scheduling Error
    #[inline(always)]
    pub fn lcse(&mut self) -> LCSE_W<'_, MTLESTCRrs> {
        LCSE_W::new(self, 6)
    }
    ///Bits 8:10 - Time Interval Left Shift Amount
    #[inline(always)]
    pub fn tils(&mut self) -> TILS_W<'_, MTLESTCRrs> {
        TILS_W::new(self, 8)
    }
    ///Bits 12:23 - Current Time Offset Value
    #[inline(always)]
    pub fn ctov(&mut self) -> CTOV_W<'_, MTLESTCRrs> {
        CTOV_W::new(self, 12)
    }
    ///Bits 24:31 - PTP Time Offset Value
    #[inline(always)]
    pub fn ptov(&mut self) -> PTOV_W<'_, MTLESTCRrs> {
        PTOV_W::new(self, 24)
    }
}
/**EST Control Register

You can [`read`](crate::Reg::read) this register and get [`mtlestcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLESTCR)*/
pub struct MTLESTCRrs;
impl crate::RegisterSpec for MTLESTCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestcr::R`](R) reader structure
impl crate::Readable for MTLESTCRrs {}
///`write(|w| ..)` method takes [`mtlestcr::W`](W) writer structure
impl crate::Writable for MTLESTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTCR to value 0
impl crate::Resettable for MTLESTCRrs {}
