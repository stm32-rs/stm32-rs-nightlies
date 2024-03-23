#[doc = "Register `WPCR0` reader"]
pub type R = crate::R<WPCR0rs>;
#[doc = "Register `WPCR0` writer"]
pub type W = crate::W<WPCR0rs>;
#[doc = "Field `UIX4` reader - UIX4"]
pub type UIX4_R = crate::FieldReader;
#[doc = "Field `UIX4` writer - UIX4"]
pub type UIX4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SWCL` reader - SWCL"]
pub type SWCL_R = crate::BitReader;
#[doc = "Field `SWCL` writer - SWCL"]
pub type SWCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL0` reader - SWDL0"]
pub type SWDL0_R = crate::BitReader;
#[doc = "Field `SWDL0` writer - SWDL0"]
pub type SWDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL1` reader - SWDL1"]
pub type SWDL1_R = crate::BitReader;
#[doc = "Field `SWDL1` writer - SWDL1"]
pub type SWDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICL` reader - HSICL"]
pub type HSICL_R = crate::BitReader;
#[doc = "Field `HSICL` writer - HSICL"]
pub type HSICL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDL0` reader - HSIDL0"]
pub type HSIDL0_R = crate::BitReader;
#[doc = "Field `HSIDL0` writer - HSIDL0"]
pub type HSIDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDL1` reader - HSIDL1"]
pub type HSIDL1_R = crate::BitReader;
#[doc = "Field `HSIDL1` writer - HSIDL1"]
pub type HSIDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMCL` reader - FTXSMCL"]
pub type FTXSMCL_R = crate::BitReader;
#[doc = "Field `FTXSMCL` writer - FTXSMCL"]
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMDL` reader - FTXSMDL"]
pub type FTXSMDL_R = crate::BitReader;
#[doc = "Field `FTXSMDL` writer - FTXSMDL"]
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDOFFDL` reader - CDOFFDL"]
pub type CDOFFDL_R = crate::BitReader;
#[doc = "Field `CDOFFDL` writer - CDOFFDL"]
pub type CDOFFDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDDL` reader - TDDL"]
pub type TDDL_R = crate::BitReader;
#[doc = "Field `TDDL` writer - TDDL"]
pub type TDDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    #[must_use]
    pub fn uix4(&mut self) -> UIX4_W<WPCR0rs> {
        UIX4_W::new(self, 0)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    #[must_use]
    pub fn hsicl(&mut self) -> HSICL_W<WPCR0rs> {
        HSICL_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl0(&mut self) -> HSIDL0_W<WPCR0rs> {
        HSIDL0_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl1(&mut self) -> HSIDL1_W<WPCR0rs> {
        HSIDL1_W::new(self, 11)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    #[must_use]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<WPCR0rs> {
        CDOFFDL_W::new(self, 14)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    #[must_use]
    pub fn tddl(&mut self) -> TDDL_W<WPCR0rs> {
        TDDL_W::new(self, 16)
    }
}
#[doc = "DSI wrapper PHY configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR0rs;
impl crate::RegisterSpec for WPCR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr0::R`](R) reader structure"]
impl crate::Readable for WPCR0rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr0::W`](W) writer structure"]
impl crate::Writable for WPCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR0 to value 0"]
impl crate::Resettable for WPCR0rs {
    const RESET_VALUE: u32 = 0;
}
