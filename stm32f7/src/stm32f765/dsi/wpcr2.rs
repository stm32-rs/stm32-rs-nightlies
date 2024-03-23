#[doc = "Register `WPCR2` reader"]
pub type R = crate::R<WPCR2rs>;
#[doc = "Register `WPCR2` writer"]
pub type W = crate::W<WPCR2rs>;
#[doc = "Field `HSTXDCL` reader - High-Speed Transmission Delay on Clock Lane"]
pub type HSTXDCL_R = crate::FieldReader;
#[doc = "Field `HSTXDCL` writer - High-Speed Transmission Delay on Clock Lane"]
pub type HSTXDCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSTXDLL` reader - High-Speed Transmission Delay on Data Lanes"]
pub type HSTXDLL_R = crate::FieldReader;
#[doc = "Field `HSTXDLL` writer - High-Speed Transmission Delay on Data Lanes"]
pub type HSTXDLL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPSRCL` reader - Low-Power transmission Slew Rate Compensation on Clock Lane"]
pub type LPSRCL_R = crate::FieldReader;
#[doc = "Field `LPSRCL` writer - Low-Power transmission Slew Rate Compensation on Clock Lane"]
pub type LPSRCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPSRDL` reader - Low-Power transmission Slew Rate Compensation on Data Lanes"]
pub type LPSRDL_R = crate::FieldReader;
#[doc = "Field `LPSRDL` writer - Low-Power transmission Slew Rate Compensation on Data Lanes"]
pub type LPSRDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCC` reader - SDD Control"]
pub type SDCC_R = crate::BitReader;
#[doc = "Field `SDCC` writer - SDD Control"]
pub type SDCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRCCL` reader - High-Speed Transmission Slew Rate Control on Clock Lane"]
pub type HSTXSRCCL_R = crate::FieldReader;
#[doc = "Field `HSTXSRCCL` writer - High-Speed Transmission Slew Rate Control on Clock Lane"]
pub type HSTXSRCCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSTXSRCDL` reader - High-Speed Transmission Slew Rate Control on Data Lanes"]
pub type HSTXSRCDL_R = crate::FieldReader;
#[doc = "Field `HSTXSRCDL` writer - High-Speed Transmission Slew Rate Control on Data Lanes"]
pub type HSTXSRCDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLPRXLPM` reader - Forces LP Receiver in Low-Power Mode"]
pub type FLPRXLPM_R = crate::BitReader;
#[doc = "Field `FLPRXLPM` writer - Forces LP Receiver in Low-Power Mode"]
pub type FLPRXLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXFT` reader - Low-Power RX low-pass Filtering Tuning"]
pub type LPRXFT_R = crate::FieldReader;
#[doc = "Field `LPRXFT` writer - Low-Power RX low-pass Filtering Tuning"]
pub type LPRXFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline(always)]
    pub fn hstxdll(&self) -> HSTXDLL_R {
        HSTXDLL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline(always)]
    pub fn lpsrcl(&self) -> LPSRCL_R {
        LPSRCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline(always)]
    pub fn lpsrdl(&self) -> LPSRDL_R {
        LPSRDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline(always)]
    pub fn sdcc(&self) -> SDCC_R {
        SDCC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline(always)]
    #[must_use]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W<WPCR2rs> {
        HSTXDCL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline(always)]
    #[must_use]
    pub fn hstxdll(&mut self) -> HSTXDLL_W<WPCR2rs> {
        HSTXDLL_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline(always)]
    #[must_use]
    pub fn lpsrcl(&mut self) -> LPSRCL_W<WPCR2rs> {
        LPSRCL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline(always)]
    #[must_use]
    pub fn lpsrdl(&mut self) -> LPSRDL_W<WPCR2rs> {
        LPSRDL_W::new(self, 8)
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline(always)]
    #[must_use]
    pub fn sdcc(&mut self) -> SDCC_W<WPCR2rs> {
        SDCC_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W<WPCR2rs> {
        HSTXSRCCL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W<WPCR2rs> {
        HSTXSRCDL_W::new(self, 18)
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W<WPCR2rs> {
        FLPRXLPM_W::new(self, 22)
    }
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn lprxft(&mut self) -> LPRXFT_W<WPCR2rs> {
        LPRXFT_W::new(self, 25)
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR2rs;
impl crate::RegisterSpec for WPCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr2::R`](R) reader structure"]
impl crate::Readable for WPCR2rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr2::W`](W) writer structure"]
impl crate::Writable for WPCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR2 to value 0"]
impl crate::Resettable for WPCR2rs {
    const RESET_VALUE: u32 = 0;
}
