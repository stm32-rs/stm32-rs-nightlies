///Register `BR_RSTSCLRR` reader
pub type R = crate::R<BR_RSTSCLRRrs>;
///Register `BR_RSTSCLRR` writer
pub type W = crate::W<BR_RSTSCLRRrs>;
///Field `PORRSTF` reader - PORRSTF
pub type PORRSTF_R = crate::BitReader;
///Field `PORRSTF` writer - PORRSTF
pub type PORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BORRSTF` reader - BORRSTF
pub type BORRSTF_R = crate::BitReader;
///Field `BORRSTF` writer - BORRSTF
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PADRSTF` reader - PADRSTF
pub type PADRSTF_R = crate::BitReader;
///Field `PADRSTF` writer - PADRSTF
pub type PADRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCSSRSTF` reader - HCSSRSTF
pub type HCSSRSTF_R = crate::BitReader;
///Field `HCSSRSTF` writer - HCSSRSTF
pub type HCSSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCORERSTF` reader - VCORERSTF
pub type VCORERSTF_R = crate::BitReader;
///Field `VCORERSTF` writer - VCORERSTF
pub type VCORERSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPSYSRSTF` reader - MPSYSRSTF
pub type MPSYSRSTF_R = crate::BitReader;
///Field `MPSYSRSTF` writer - MPSYSRSTF
pub type MPSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCSYSRSTF` reader - MCSYSRSTF
pub type MCSYSRSTF_R = crate::BitReader;
///Field `MCSYSRSTF` writer - MCSYSRSTF
pub type MCSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG1RSTF` reader - IWDG1RSTF
pub type IWDG1RSTF_R = crate::BitReader;
///Field `IWDG1RSTF` writer - IWDG1RSTF
pub type IWDG1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG2RSTF` reader - IWDG2RSTF
pub type IWDG2RSTF_R = crate::BitReader;
///Field `IWDG2RSTF` writer - IWDG2RSTF
pub type IWDG2RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPUP0RSTF` reader - MPUP0RSTF
pub type MPUP0RSTF_R = crate::BitReader;
///Field `MPUP0RSTF` writer - MPUP0RSTF
pub type MPUP0RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPUP1RSTF` reader - MPUP1RSTF
pub type MPUP1RSTF_R = crate::BitReader;
///Field `MPUP1RSTF` writer - MPUP1RSTF
pub type MPUP1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PORRSTF
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BORRSTF
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PADRSTF
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HCSSRSTF
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VCORERSTF
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - MPSYSRSTF
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MCSYSRSTF
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IWDG1RSTF
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IWDG2RSTF
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - MPUP0RSTF
    #[inline(always)]
    pub fn mpup0rstf(&self) -> MPUP0RSTF_R {
        MPUP0RSTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MPUP1RSTF
    #[inline(always)]
    pub fn mpup1rstf(&self) -> MPUP1RSTF_R {
        MPUP1RSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BR_RSTSCLRR")
            .field("porrstf", &self.porrstf())
            .field("borrstf", &self.borrstf())
            .field("padrstf", &self.padrstf())
            .field("hcssrstf", &self.hcssrstf())
            .field("vcorerstf", &self.vcorerstf())
            .field("mpsysrstf", &self.mpsysrstf())
            .field("mcsysrstf", &self.mcsysrstf())
            .field("iwdg1rstf", &self.iwdg1rstf())
            .field("iwdg2rstf", &self.iwdg2rstf())
            .field("mpup0rstf", &self.mpup0rstf())
            .field("mpup1rstf", &self.mpup1rstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - PORRSTF
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<BR_RSTSCLRRrs> {
        PORRSTF_W::new(self, 0)
    }
    ///Bit 1 - BORRSTF
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<BR_RSTSCLRRrs> {
        BORRSTF_W::new(self, 1)
    }
    ///Bit 2 - PADRSTF
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W<BR_RSTSCLRRrs> {
        PADRSTF_W::new(self, 2)
    }
    ///Bit 3 - HCSSRSTF
    #[inline(always)]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W<BR_RSTSCLRRrs> {
        HCSSRSTF_W::new(self, 3)
    }
    ///Bit 4 - VCORERSTF
    #[inline(always)]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W<BR_RSTSCLRRrs> {
        VCORERSTF_W::new(self, 4)
    }
    ///Bit 6 - MPSYSRSTF
    #[inline(always)]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W<BR_RSTSCLRRrs> {
        MPSYSRSTF_W::new(self, 6)
    }
    ///Bit 7 - MCSYSRSTF
    #[inline(always)]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W<BR_RSTSCLRRrs> {
        MCSYSRSTF_W::new(self, 7)
    }
    ///Bit 8 - IWDG1RSTF
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<BR_RSTSCLRRrs> {
        IWDG1RSTF_W::new(self, 8)
    }
    ///Bit 9 - IWDG2RSTF
    #[inline(always)]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W<BR_RSTSCLRRrs> {
        IWDG2RSTF_W::new(self, 9)
    }
    ///Bit 13 - MPUP0RSTF
    #[inline(always)]
    pub fn mpup0rstf(&mut self) -> MPUP0RSTF_W<BR_RSTSCLRRrs> {
        MPUP0RSTF_W::new(self, 13)
    }
    ///Bit 14 - MPUP1RSTF
    #[inline(always)]
    pub fn mpup1rstf(&mut self) -> MPUP1RSTF_W<BR_RSTSCLRRrs> {
        MPUP1RSTF_W::new(self, 14)
    }
}
/**This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`br_rstsclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br_rstsclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:BR_RSTSCLRR)*/
pub struct BR_RSTSCLRRrs;
impl crate::RegisterSpec for BR_RSTSCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`br_rstsclrr::R`](R) reader structure
impl crate::Readable for BR_RSTSCLRRrs {}
///`write(|w| ..)` method takes [`br_rstsclrr::W`](W) writer structure
impl crate::Writable for BR_RSTSCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BR_RSTSCLRR to value 0x15
impl crate::Resettable for BR_RSTSCLRRrs {
    const RESET_VALUE: u32 = 0x15;
}
