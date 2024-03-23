#[doc = "Register `RCC_MP_RSTSSETR` reader"]
pub type R = crate::R<RCC_MP_RSTSSETRrs>;
#[doc = "Register `RCC_MP_RSTSSETR` writer"]
pub type W = crate::W<RCC_MP_RSTSSETRrs>;
#[doc = "Field `PORRSTF` reader - PORRSTF"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` writer - PORRSTF"]
pub type PORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORRSTF` reader - BORRSTF"]
pub type BORRSTF_R = crate::BitReader;
#[doc = "Field `BORRSTF` writer - BORRSTF"]
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADRSTF` reader - PADRSTF"]
pub type PADRSTF_R = crate::BitReader;
#[doc = "Field `PADRSTF` writer - PADRSTF"]
pub type PADRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCSSRSTF` reader - HCSSRSTF"]
pub type HCSSRSTF_R = crate::BitReader;
#[doc = "Field `HCSSRSTF` writer - HCSSRSTF"]
pub type HCSSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCORERSTF` reader - VCORERSTF"]
pub type VCORERSTF_R = crate::BitReader;
#[doc = "Field `VCORERSTF` writer - VCORERSTF"]
pub type VCORERSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPSYSRSTF` reader - MPSYSRSTF"]
pub type MPSYSRSTF_R = crate::BitReader;
#[doc = "Field `MPSYSRSTF` writer - MPSYSRSTF"]
pub type MPSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCSYSRSTF` reader - MCSYSRSTF"]
pub type MCSYSRSTF_R = crate::BitReader;
#[doc = "Field `MCSYSRSTF` writer - MCSYSRSTF"]
pub type MCSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG1RSTF` reader - IWDG1RSTF"]
pub type IWDG1RSTF_R = crate::BitReader;
#[doc = "Field `IWDG1RSTF` writer - IWDG1RSTF"]
pub type IWDG1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG2RSTF` reader - IWDG2RSTF"]
pub type IWDG2RSTF_R = crate::BitReader;
#[doc = "Field `IWDG2RSTF` writer - IWDG2RSTF"]
pub type IWDG2RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STDBYRSTF` reader - STDBYRSTF"]
pub type STDBYRSTF_R = crate::BitReader;
#[doc = "Field `STDBYRSTF` writer - STDBYRSTF"]
pub type STDBYRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTDBYRSTF` reader - CSTDBYRSTF"]
pub type CSTDBYRSTF_R = crate::BitReader;
#[doc = "Field `CSTDBYRSTF` writer - CSTDBYRSTF"]
pub type CSTDBYRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPUP0RSTF` reader - MPUP0RSTF"]
pub type MPUP0RSTF_R = crate::BitReader;
#[doc = "Field `MPUP0RSTF` writer - MPUP0RSTF"]
pub type MPUP0RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPUP1RSTF` reader - MPUP1RSTF"]
pub type MPUP1RSTF_R = crate::BitReader;
#[doc = "Field `MPUP1RSTF` writer - MPUP1RSTF"]
pub type MPUP1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE` reader - SPARE"]
pub type SPARE_R = crate::BitReader;
#[doc = "Field `SPARE` writer - SPARE"]
pub type SPARE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - STDBYRSTF"]
    #[inline(always)]
    pub fn stdbyrstf(&self) -> STDBYRSTF_R {
        STDBYRSTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CSTDBYRSTF"]
    #[inline(always)]
    pub fn cstdbyrstf(&self) -> CSTDBYRSTF_R {
        CSTDBYRSTF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MPUP0RSTF"]
    #[inline(always)]
    pub fn mpup0rstf(&self) -> MPUP0RSTF_R {
        MPUP0RSTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MPUP1RSTF"]
    #[inline(always)]
    pub fn mpup1rstf(&self) -> MPUP1RSTF_R {
        MPUP1RSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPARE"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<RCC_MP_RSTSSETRrs> {
        PORRSTF_W::new(self, 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<RCC_MP_RSTSSETRrs> {
        BORRSTF_W::new(self, 1)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn padrstf(&mut self) -> PADRSTF_W<RCC_MP_RSTSSETRrs> {
        PADRSTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W<RCC_MP_RSTSSETRrs> {
        HCSSRSTF_W::new(self, 3)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    #[must_use]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W<RCC_MP_RSTSSETRrs> {
        VCORERSTF_W::new(self, 4)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W<RCC_MP_RSTSSETRrs> {
        MPSYSRSTF_W::new(self, 6)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W<RCC_MP_RSTSSETRrs> {
        MCSYSRSTF_W::new(self, 7)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<RCC_MP_RSTSSETRrs> {
        IWDG1RSTF_W::new(self, 8)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W<RCC_MP_RSTSSETRrs> {
        IWDG2RSTF_W::new(self, 9)
    }
    #[doc = "Bit 11 - STDBYRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn stdbyrstf(&mut self) -> STDBYRSTF_W<RCC_MP_RSTSSETRrs> {
        STDBYRSTF_W::new(self, 11)
    }
    #[doc = "Bit 12 - CSTDBYRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn cstdbyrstf(&mut self) -> CSTDBYRSTF_W<RCC_MP_RSTSSETRrs> {
        CSTDBYRSTF_W::new(self, 12)
    }
    #[doc = "Bit 13 - MPUP0RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mpup0rstf(&mut self) -> MPUP0RSTF_W<RCC_MP_RSTSSETRrs> {
        MPUP0RSTF_W::new(self, 13)
    }
    #[doc = "Bit 14 - MPUP1RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mpup1rstf(&mut self) -> MPUP1RSTF_W<RCC_MP_RSTSSETRrs> {
        MPUP1RSTF_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPARE"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<RCC_MP_RSTSSETRrs> {
        SPARE_W::new(self, 15)
    }
}
#[doc = "This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_rstssetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_rstssetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_RSTSSETRrs;
impl crate::RegisterSpec for RCC_MP_RSTSSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_rstssetr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_RSTSSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_rstssetr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_RSTSSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_RSTSSETR to value 0"]
impl crate::Resettable for RCC_MP_RSTSSETRrs {
    const RESET_VALUE: u32 = 0;
}
