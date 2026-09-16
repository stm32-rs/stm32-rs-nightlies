///Register `WPCR1` reader
pub type R = crate::R<WPCR1rs>;
///Register `WPCR1` writer
pub type W = crate::W<WPCR1rs>;
///Field `HSTXDCL` reader - High-Speed Transmission Delay on Clock Lane
pub type HSTXDCL_R = crate::FieldReader;
///Field `HSTXDCL` writer - High-Speed Transmission Delay on Clock Lane
pub type HSTXDCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSTXDLL` reader - High-Speed Transmission Delay on Data Lanes
pub type HSTXDLL_R = crate::FieldReader;
///Field `HSTXDLL` writer - High-Speed Transmission Delay on Data Lanes
pub type HSTXDLL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPSRCL` reader - Low-Power transmission Slew Rate Compensation on Clock Lane
pub type LPSRCL_R = crate::FieldReader;
///Field `LPSRCL` writer - Low-Power transmission Slew Rate Compensation on Clock Lane
pub type LPSRCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPSRDL` reader - Low-Power transmission Slew Rate Compensation on Data Lanes
pub type LPSRDL_R = crate::FieldReader;
///Field `LPSRDL` writer - Low-Power transmission Slew Rate Compensation on Data Lanes
pub type LPSRDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDCC` reader - SDD Control
pub type SDCC_R = crate::BitReader;
///Field `SDCC` writer - SDD Control
pub type SDCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTXSRCCL` reader - High-Speed Transmission Slew Rate Control on Clock Lane
pub type HSTXSRCCL_R = crate::FieldReader;
///Field `HSTXSRCCL` writer - High-Speed Transmission Slew Rate Control on Clock Lane
pub type HSTXSRCCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSTXSRCDL` reader - High-Speed Transmission Slew Rate Control on Data Lanes
pub type HSTXSRCDL_R = crate::FieldReader;
///Field `HSTXSRCDL` writer - High-Speed Transmission Slew Rate Control on Data Lanes
pub type HSTXSRCDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FLPRXLPM` reader - Forces LP Receiver in Low-Power Mode
pub type FLPRXLPM_R = crate::BitReader;
///Field `FLPRXLPM` writer - Forces LP Receiver in Low-Power Mode
pub type FLPRXLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPRXFT` reader - Low-Power RX low-pass Filtering Tuning
pub type LPRXFT_R = crate::FieldReader;
///Field `LPRXFT` writer - Low-Power RX low-pass Filtering Tuning
pub type LPRXFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - High-Speed Transmission Delay on Clock Lane
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - High-Speed Transmission Delay on Data Lanes
    #[inline(always)]
    pub fn hstxdll(&self) -> HSTXDLL_R {
        HSTXDLL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane
    #[inline(always)]
    pub fn lpsrcl(&self) -> LPSRCL_R {
        LPSRCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes
    #[inline(always)]
    pub fn lpsrdl(&self) -> LPSRDL_R {
        LPSRDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 12 - SDD Control
    #[inline(always)]
    pub fn sdcc(&self) -> SDCC_R {
        SDCC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 22 - Forces LP Receiver in Low-Power Mode
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 25:26 - Low-Power RX low-pass Filtering Tuning
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR1")
            .field("lprxft", &self.lprxft())
            .field("flprxlpm", &self.flprxlpm())
            .field("hstxsrcdl", &self.hstxsrcdl())
            .field("hstxsrccl", &self.hstxsrccl())
            .field("sdcc", &self.sdcc())
            .field("lpsrdl", &self.lpsrdl())
            .field("lpsrcl", &self.lpsrcl())
            .field("hstxdll", &self.hstxdll())
            .field("hstxdcl", &self.hstxdcl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - High-Speed Transmission Delay on Clock Lane
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W<'_, WPCR1rs> {
        HSTXDCL_W::new(self, 0)
    }
    ///Bits 2:3 - High-Speed Transmission Delay on Data Lanes
    #[inline(always)]
    pub fn hstxdll(&mut self) -> HSTXDLL_W<'_, WPCR1rs> {
        HSTXDLL_W::new(self, 2)
    }
    ///Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane
    #[inline(always)]
    pub fn lpsrcl(&mut self) -> LPSRCL_W<'_, WPCR1rs> {
        LPSRCL_W::new(self, 6)
    }
    ///Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes
    #[inline(always)]
    pub fn lpsrdl(&mut self) -> LPSRDL_W<'_, WPCR1rs> {
        LPSRDL_W::new(self, 8)
    }
    ///Bit 12 - SDD Control
    #[inline(always)]
    pub fn sdcc(&mut self) -> SDCC_W<'_, WPCR1rs> {
        SDCC_W::new(self, 12)
    }
    ///Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W<'_, WPCR1rs> {
        HSTXSRCCL_W::new(self, 16)
    }
    ///Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W<'_, WPCR1rs> {
        HSTXSRCDL_W::new(self, 18)
    }
    ///Bit 22 - Forces LP Receiver in Low-Power Mode
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W<'_, WPCR1rs> {
        FLPRXLPM_W::new(self, 22)
    }
    ///Bits 25:26 - Low-Power RX low-pass Filtering Tuning
    #[inline(always)]
    pub fn lprxft(&mut self) -> LPRXFT_W<'_, WPCR1rs> {
        LPRXFT_W::new(self, 25)
    }
}
/**DSI Wrapper PHY Configuration Register 2

You can [`read`](crate::Reg::read) this register and get [`wpcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:WPCR1)*/
pub struct WPCR1rs;
impl crate::RegisterSpec for WPCR1rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr1::R`](R) reader structure
impl crate::Readable for WPCR1rs {}
///`write(|w| ..)` method takes [`wpcr1::W`](W) writer structure
impl crate::Writable for WPCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR1 to value 0
impl crate::Resettable for WPCR1rs {}
