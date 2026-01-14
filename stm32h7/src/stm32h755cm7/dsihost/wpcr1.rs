///Register `WPCR1` reader
pub type R = crate::R<WPCR1rs>;
///Register `WPCR1` writer
pub type W = crate::W<WPCR1rs>;
///Field `HSTXDCL` reader - High-speed transmission delay on clock lane
pub type HSTXDCL_R = crate::FieldReader;
///Field `HSTXDCL` writer - High-speed transmission delay on clock lane
pub type HSTXDCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSTXDDL` reader - High-speed transmission delay on data lanes
pub type HSTXDDL_R = crate::FieldReader;
///Field `HSTXDDL` writer - High-speed transmission delay on data lanes
pub type HSTXDDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPSRCCL` reader - Low-power transmission slew-rate compensation on clock lane
pub type LPSRCCL_R = crate::FieldReader;
///Field `LPSRCCL` writer - Low-power transmission slew-rate compensation on clock lane
pub type LPSRCCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPSRCDL` reader - Low-power transmission slew-rate compensation on data lanes
pub type LPSRCDL_R = crate::FieldReader;
///Field `LPSRCDL` writer - Low-power transmission slew-rate compensation on data lanes
pub type LPSRCDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDDC` reader - SDD control
pub type SDDC_R = crate::BitReader;
///Field `SDDC` writer - SDD control
pub type SDDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTXSRCCL` reader - High-speed transmission slew-rate control on clock lane
pub type HSTXSRCCL_R = crate::FieldReader;
///Field `HSTXSRCCL` writer - High-speed transmission slew-rate control on clock lane
pub type HSTXSRCCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSTXSRCDL` reader - High-speed transmission slew-rate control on data lanes
pub type HSTXSRCDL_R = crate::FieldReader;
///Field `HSTXSRCDL` writer - High-speed transmission slew-rate control on data lanes
pub type HSTXSRCDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FLPRXLPM` reader - Forces LP receiver in low-power mode
pub type FLPRXLPM_R = crate::BitReader;
///Field `FLPRXLPM` writer - Forces LP receiver in low-power mode
pub type FLPRXLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPRXFT` reader - Low-power RX low-pass filtering tuning
pub type LPRXFT_R = crate::FieldReader;
///Field `LPRXFT` writer - Low-power RX low-pass filtering tuning
pub type LPRXFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - High-speed transmission delay on clock lane
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - High-speed transmission delay on data lanes
    #[inline(always)]
    pub fn hstxddl(&self) -> HSTXDDL_R {
        HSTXDDL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Low-power transmission slew-rate compensation on clock lane
    #[inline(always)]
    pub fn lpsrccl(&self) -> LPSRCCL_R {
        LPSRCCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Low-power transmission slew-rate compensation on data lanes
    #[inline(always)]
    pub fn lpsrcdl(&self) -> LPSRCDL_R {
        LPSRCDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 12 - SDD control
    #[inline(always)]
    pub fn sddc(&self) -> SDDC_R {
        SDDC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:17 - High-speed transmission slew-rate control on clock lane
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - High-speed transmission slew-rate control on data lanes
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 22 - Forces LP receiver in low-power mode
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 25:26 - Low-power RX low-pass filtering tuning
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR1")
            .field("hstxdcl", &self.hstxdcl())
            .field("hstxddl", &self.hstxddl())
            .field("lpsrccl", &self.lpsrccl())
            .field("lpsrcdl", &self.lpsrcdl())
            .field("sddc", &self.sddc())
            .field("hstxsrccl", &self.hstxsrccl())
            .field("hstxsrcdl", &self.hstxsrcdl())
            .field("flprxlpm", &self.flprxlpm())
            .field("lprxft", &self.lprxft())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - High-speed transmission delay on clock lane
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W<'_, WPCR1rs> {
        HSTXDCL_W::new(self, 0)
    }
    ///Bits 2:3 - High-speed transmission delay on data lanes
    #[inline(always)]
    pub fn hstxddl(&mut self) -> HSTXDDL_W<'_, WPCR1rs> {
        HSTXDDL_W::new(self, 2)
    }
    ///Bits 6:7 - Low-power transmission slew-rate compensation on clock lane
    #[inline(always)]
    pub fn lpsrccl(&mut self) -> LPSRCCL_W<'_, WPCR1rs> {
        LPSRCCL_W::new(self, 6)
    }
    ///Bits 8:9 - Low-power transmission slew-rate compensation on data lanes
    #[inline(always)]
    pub fn lpsrcdl(&mut self) -> LPSRCDL_W<'_, WPCR1rs> {
        LPSRCDL_W::new(self, 8)
    }
    ///Bit 12 - SDD control
    #[inline(always)]
    pub fn sddc(&mut self) -> SDDC_W<'_, WPCR1rs> {
        SDDC_W::new(self, 12)
    }
    ///Bits 16:17 - High-speed transmission slew-rate control on clock lane
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W<'_, WPCR1rs> {
        HSTXSRCCL_W::new(self, 16)
    }
    ///Bits 18:19 - High-speed transmission slew-rate control on data lanes
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W<'_, WPCR1rs> {
        HSTXSRCDL_W::new(self, 18)
    }
    ///Bit 22 - Forces LP receiver in low-power mode
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W<'_, WPCR1rs> {
        FLPRXLPM_W::new(self, 22)
    }
    ///Bits 25:26 - Low-power RX low-pass filtering tuning
    #[inline(always)]
    pub fn lprxft(&mut self) -> LPRXFT_W<'_, WPCR1rs> {
        LPRXFT_W::new(self, 25)
    }
}
/**This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).

You can [`read`](crate::Reg::read) this register and get [`wpcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#DSIHOST:WPCR1)*/
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
