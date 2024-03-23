#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `LSION` reader - LSI oscillator enable"]
pub type LSION_R = crate::BitReader;
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
pub type LSIRDY_R = crate::BitReader;
#[doc = "Field `MSISRANGE` reader - SI range after Standby mode"]
pub type MSISRANGE_R = crate::FieldReader;
#[doc = "Field `MSISRANGE` writer - SI range after Standby mode"]
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIREWALLRSTF` reader - Firewall reset flag"]
pub type FIREWALLRSTF_R = crate::BitReader;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OBLRSTF_R = crate::BitReader;
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub type PINRSTF_R = crate::BitReader;
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub type BORRSTF_R = crate::BitReader;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader;
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader;
#[doc = "Field `LPWRSTF` reader - Low-power reset flag"]
pub type LPWRSTF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - SI range after Standby mode"]
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Firewall reset flag"]
    #[inline(always)]
    pub fn firewallrstf(&self) -> FIREWALLRSTF_R {
        FIREWALLRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - SI range after Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<CSRrs> {
        MSISRANGE_W::new(self, 8)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0600"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0600;
}
