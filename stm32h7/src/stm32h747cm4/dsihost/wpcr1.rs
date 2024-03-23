#[doc = "Register `WPCR1` reader"]
pub type R = crate::R<WPCR1rs>;
#[doc = "Register `WPCR1` writer"]
pub type W = crate::W<WPCR1rs>;
#[doc = "Field `HSTXDCL` reader - High-speed transmission delay on clock lane"]
pub type HSTXDCL_R = crate::FieldReader;
#[doc = "Field `HSTXDCL` writer - High-speed transmission delay on clock lane"]
pub type HSTXDCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSTXDDL` reader - High-speed transmission delay on data lanes"]
pub type HSTXDDL_R = crate::FieldReader;
#[doc = "Field `HSTXDDL` writer - High-speed transmission delay on data lanes"]
pub type HSTXDDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPSRCCL` reader - Low-power transmission slew-rate compensation on clock lane"]
pub type LPSRCCL_R = crate::FieldReader;
#[doc = "Field `LPSRCCL` writer - Low-power transmission slew-rate compensation on clock lane"]
pub type LPSRCCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPSRCDL` reader - Low-power transmission slew-rate compensation on data lanes"]
pub type LPSRCDL_R = crate::FieldReader;
#[doc = "Field `LPSRCDL` writer - Low-power transmission slew-rate compensation on data lanes"]
pub type LPSRCDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDDC` reader - SDD control"]
pub type SDDC_R = crate::BitReader;
#[doc = "Field `SDDC` writer - SDD control"]
pub type SDDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRCCL` reader - High-speed transmission slew-rate control on clock lane"]
pub type HSTXSRCCL_R = crate::FieldReader;
#[doc = "Field `HSTXSRCCL` writer - High-speed transmission slew-rate control on clock lane"]
pub type HSTXSRCCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSTXSRCDL` reader - High-speed transmission slew-rate control on data lanes"]
pub type HSTXSRCDL_R = crate::FieldReader;
#[doc = "Field `HSTXSRCDL` writer - High-speed transmission slew-rate control on data lanes"]
pub type HSTXSRCDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLPRXLPM` reader - Forces LP receiver in low-power mode"]
pub type FLPRXLPM_R = crate::BitReader;
#[doc = "Field `FLPRXLPM` writer - Forces LP receiver in low-power mode"]
pub type FLPRXLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXFT` reader - Low-power RX low-pass filtering tuning"]
pub type LPRXFT_R = crate::FieldReader;
#[doc = "Field `LPRXFT` writer - Low-power RX low-pass filtering tuning"]
pub type LPRXFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&self) -> HSTXDDL_R {
        HSTXDDL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&self) -> LPSRCCL_R {
        LPSRCCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&self) -> LPSRCDL_R {
        LPSRCDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&self) -> SDDC_R {
        SDDC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W<WPCR1rs> {
        HSTXDCL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn hstxddl(&mut self) -> HSTXDDL_W<WPCR1rs> {
        HSTXDDL_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn lpsrccl(&mut self) -> LPSRCCL_W<WPCR1rs> {
        LPSRCCL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn lpsrcdl(&mut self) -> LPSRCDL_W<WPCR1rs> {
        LPSRCDL_W::new(self, 8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    #[must_use]
    pub fn sddc(&mut self) -> SDDC_W<WPCR1rs> {
        SDDC_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W<WPCR1rs> {
        HSTXSRCCL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W<WPCR1rs> {
        HSTXSRCDL_W::new(self, 18)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W<WPCR1rs> {
        FLPRXLPM_W::new(self, 22)
    }
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    #[must_use]
    pub fn lprxft(&mut self) -> LPRXFT_W<WPCR1rs> {
        LPRXFT_W::new(self, 25)
    }
}
#[doc = "DSI wrapper PHY configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR1rs;
impl crate::RegisterSpec for WPCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr1::R`](R) reader structure"]
impl crate::Readable for WPCR1rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr1::W`](W) writer structure"]
impl crate::Writable for WPCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1rs {
    const RESET_VALUE: u32 = 0;
}
