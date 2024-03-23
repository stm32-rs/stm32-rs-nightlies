#[doc = "Register `CSR1` reader"]
pub type R = crate::R<CSR1rs>;
#[doc = "Register `CSR1` writer"]
pub type W = crate::W<CSR1rs>;
#[doc = "Field `WUIF` reader - Wakeup internal flag"]
pub type WUIF_R = crate::BitReader;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `PVDO` reader - PVD output"]
pub type PVDO_R = crate::BitReader;
#[doc = "Field `BRR` reader - Backup regulator ready"]
pub type BRR_R = crate::BitReader;
#[doc = "Field `EIWUP` reader - Enable internal wakeup"]
pub type EIWUP_R = crate::BitReader;
#[doc = "Field `EIWUP` writer - Enable internal wakeup"]
pub type EIWUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRE` reader - Backup regulator enable"]
pub type BRE_R = crate::BitReader;
#[doc = "Field `BRE` writer - Backup regulator enable"]
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit"]
pub type VOSRDY_R = crate::BitReader;
#[doc = "Field `VOSRDY` writer - Regulator voltage scaling output selection ready bit"]
pub type VOSRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODRDY` reader - Over-drive mode ready"]
pub type ODRDY_R = crate::BitReader;
#[doc = "Field `ODRDY` writer - Over-drive mode ready"]
pub type ODRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODSWRDY` reader - Over-drive mode switching ready"]
pub type ODSWRDY_R = crate::BitReader;
#[doc = "Field `ODSWRDY` writer - Over-drive mode switching ready"]
pub type ODSWRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRDY` reader - Under-drive ready flag"]
pub type UDRDY_R = crate::FieldReader;
#[doc = "Field `UDRDY` writer - Under-drive ready flag"]
pub type UDRDY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Wakeup internal flag"]
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Backup regulator ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable internal wakeup"]
    #[inline(always)]
    pub fn eiwup(&self) -> EIWUP_R {
        EIWUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Over-drive mode ready"]
    #[inline(always)]
    pub fn odrdy(&self) -> ODRDY_R {
        ODRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Over-drive mode switching ready"]
    #[inline(always)]
    pub fn odswrdy(&self) -> ODSWRDY_R {
        ODSWRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Under-drive ready flag"]
    #[inline(always)]
    pub fn udrdy(&self) -> UDRDY_R {
        UDRDY_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Enable internal wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn eiwup(&mut self) -> EIWUP_W<CSR1rs> {
        EIWUP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn bre(&mut self) -> BRE_W<CSR1rs> {
        BRE_W::new(self, 9)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    #[must_use]
    pub fn vosrdy(&mut self) -> VOSRDY_W<CSR1rs> {
        VOSRDY_W::new(self, 14)
    }
    #[doc = "Bit 16 - Over-drive mode ready"]
    #[inline(always)]
    #[must_use]
    pub fn odrdy(&mut self) -> ODRDY_W<CSR1rs> {
        ODRDY_W::new(self, 16)
    }
    #[doc = "Bit 17 - Over-drive mode switching ready"]
    #[inline(always)]
    #[must_use]
    pub fn odswrdy(&mut self) -> ODSWRDY_W<CSR1rs> {
        ODSWRDY_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Under-drive ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn udrdy(&mut self) -> UDRDY_W<CSR1rs> {
        UDRDY_W::new(self, 18)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR1rs;
impl crate::RegisterSpec for CSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for CSR1rs {}
#[doc = "`write(|w| ..)` method takes [`csr1::W`](W) writer structure"]
impl crate::Writable for CSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for CSR1rs {
    const RESET_VALUE: u32 = 0;
}
