#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1"]
pub type EWUP1_R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1"]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2"]
pub type EWUP2_R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2"]
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3"]
pub type EWUP3_R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3"]
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4"]
pub type EWUP4_R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4"]
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5"]
pub type EWUP5_R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5"]
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub type RRS_R = crate::FieldReader;
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub type RRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration"]
pub type APC_R = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration"]
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPMEN` reader - ULPMEN"]
pub type ULPMEN_R = crate::BitReader;
#[doc = "Field `ULPMEN` writer - ULPMEN"]
pub type ULPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD_STDBY` reader - UCPD_STDBY"]
pub type UCPD_STDBY_R = crate::BitReader;
#[doc = "Field `UCPD_STDBY` writer - UCPD_STDBY"]
pub type UCPD_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD_DBDIS` reader - UCPD_DBDIS"]
pub type UCPD_DBDIS_R = crate::BitReader;
#[doc = "Field `UCPD_DBDIS` writer - UCPD_DBDIS"]
pub type UCPD_DBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ULPMEN"]
    #[inline(always)]
    pub fn ulpmen(&self) -> ULPMEN_R {
        ULPMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - UCPD_STDBY"]
    #[inline(always)]
    pub fn ucpd_stdby(&self) -> UCPD_STDBY_R {
        UCPD_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UCPD_DBDIS"]
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CR3rs> {
        EWUP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CR3rs> {
        EWUP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<CR3rs> {
        EWUP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<CR3rs> {
        EWUP4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<CR3rs> {
        EWUP5_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<CR3rs> {
        RRS_W::new(self, 8)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<CR3rs> {
        APC_W::new(self, 10)
    }
    #[doc = "Bit 11 - ULPMEN"]
    #[inline(always)]
    #[must_use]
    pub fn ulpmen(&mut self) -> ULPMEN_W<CR3rs> {
        ULPMEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - UCPD_STDBY"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd_stdby(&mut self) -> UCPD_STDBY_W<CR3rs> {
        UCPD_STDBY_W::new(self, 13)
    }
    #[doc = "Bit 14 - UCPD_DBDIS"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<CR3rs> {
        UCPD_DBDIS_W::new(self, 14)
    }
}
#[doc = "Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
