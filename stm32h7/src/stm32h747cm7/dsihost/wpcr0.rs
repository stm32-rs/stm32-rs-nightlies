#[doc = "Register `WPCR0` reader"]
pub type R = crate::R<WPCR0rs>;
#[doc = "Register `WPCR0` writer"]
pub type W = crate::W<WPCR0rs>;
#[doc = "Field `UIX4` reader - Unit interval multiplied by 4"]
pub type UIX4_R = crate::FieldReader;
#[doc = "Field `UIX4` writer - Unit interval multiplied by 4"]
pub type UIX4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SWCL` reader - Swap clock lane pins"]
pub type SWCL_R = crate::BitReader;
#[doc = "Field `SWCL` writer - Swap clock lane pins"]
pub type SWCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL0` reader - Swap data lane 0 pins"]
pub type SWDL0_R = crate::BitReader;
#[doc = "Field `SWDL0` writer - Swap data lane 0 pins"]
pub type SWDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL1` reader - Swap data lane 1 pins"]
pub type SWDL1_R = crate::BitReader;
#[doc = "Field `SWDL1` writer - Swap data lane 1 pins"]
pub type SWDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICL` reader - Invert high"]
pub type HSICL_R = crate::BitReader;
#[doc = "Field `HSICL` writer - Invert high"]
pub type HSICL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDL0` reader - Invert the high-speed data signal on data lane 0"]
pub type HSIDL0_R = crate::BitReader;
#[doc = "Field `HSIDL0` writer - Invert the high-speed data signal on data lane 0"]
pub type HSIDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDL1` reader - Invert the high-speed data signal on data lane 1"]
pub type HSIDL1_R = crate::BitReader;
#[doc = "Field `HSIDL1` writer - Invert the high-speed data signal on data lane 1"]
pub type HSIDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMCL` reader - Force in TX Stop mode the clock lane"]
pub type FTXSMCL_R = crate::BitReader;
#[doc = "Field `FTXSMCL` writer - Force in TX Stop mode the clock lane"]
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMDL` reader - Force in TX Stop mode the data lanes"]
pub type FTXSMDL_R = crate::BitReader;
#[doc = "Field `FTXSMDL` writer - Force in TX Stop mode the data lanes"]
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDOFFDL` reader - Contention detection OFF on data lanes"]
pub type CDOFFDL_R = crate::BitReader;
#[doc = "Field `CDOFFDL` writer - Contention detection OFF on data lanes"]
pub type CDOFFDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDDL` reader - Turn disable data lanes"]
pub type TDDL_R = crate::BitReader;
#[doc = "Field `TDDL` writer - Turn disable data lanes"]
pub type TDDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - Pull-down enable"]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - Pull-down enable"]
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKPREPEN` reader - Custom time for tCLK"]
pub type TCLKPREPEN_R = crate::BitReader;
#[doc = "Field `TCLKPREPEN` writer - Custom time for tCLK"]
pub type TCLKPREPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKZEROEN` reader - Custom time for tCLK"]
pub type TCLKZEROEN_R = crate::BitReader;
#[doc = "Field `TCLKZEROEN` writer - Custom time for tCLK"]
pub type TCLKZEROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSPREPEN` reader - Custom time for tHS"]
pub type THSPREPEN_R = crate::BitReader;
#[doc = "Field `THSPREPEN` writer - Custom time for tHS"]
pub type THSPREPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSTRAILEN` reader - Custom time for tHS"]
pub type THSTRAILEN_R = crate::BitReader;
#[doc = "Field `THSTRAILEN` writer - Custom time for tHS"]
pub type THSTRAILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSZEROEN` reader - Custom time for tHS"]
pub type THSZEROEN_R = crate::BitReader;
#[doc = "Field `THSZEROEN` writer - Custom time for tHS"]
pub type THSZEROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLPXDEN` reader - Custom time for tLPX for data lanes enable"]
pub type TLPXDEN_R = crate::BitReader;
#[doc = "Field `TLPXDEN` writer - Custom time for tLPX for data lanes enable"]
pub type TLPXDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSEXITEN` reader - Custom time for tHS"]
pub type THSEXITEN_R = crate::BitReader;
#[doc = "Field `THSEXITEN` writer - Custom time for tHS"]
pub type THSEXITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLPXCEN` reader - Custom time for tLPX for clock lane enable"]
pub type TLPXCEN_R = crate::BitReader;
#[doc = "Field `TLPXCEN` writer - Custom time for tLPX for clock lane enable"]
pub type TLPXCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKPOSTEN` reader - Custom time for tCLK"]
pub type TCLKPOSTEN_R = crate::BitReader;
#[doc = "Field `TCLKPOSTEN` writer - Custom time for tCLK"]
pub type TCLKPOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Unit interval multiplied by 4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Swap clock lane pins"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Swap data lane 0 pins"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Swap data lane 1 pins"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Invert high"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invert the high-speed data signal on data lane 0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Invert the high-speed data signal on data lane 1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force in TX Stop mode the clock lane"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force in TX Stop mode the data lanes"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Contention detection OFF on data lanes"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Turn disable data lanes"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Custom time for tCLK"]
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Custom time for tCLK"]
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Custom time for tHS"]
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Custom time for tHS"]
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Custom time for tHS"]
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Custom time for tHS"]
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Custom time for tCLK"]
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Unit interval multiplied by 4"]
    #[inline(always)]
    #[must_use]
    pub fn uix4(&mut self) -> UIX4_W<WPCR0rs> {
        UIX4_W::new(self, 0)
    }
    #[doc = "Bit 6 - Swap clock lane pins"]
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Swap data lane 0 pins"]
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    #[doc = "Bit 8 - Swap data lane 1 pins"]
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Invert high"]
    #[inline(always)]
    #[must_use]
    pub fn hsicl(&mut self) -> HSICL_W<WPCR0rs> {
        HSICL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Invert the high-speed data signal on data lane 0"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl0(&mut self) -> HSIDL0_W<WPCR0rs> {
        HSIDL0_W::new(self, 10)
    }
    #[doc = "Bit 11 - Invert the high-speed data signal on data lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl1(&mut self) -> HSIDL1_W<WPCR0rs> {
        HSIDL1_W::new(self, 11)
    }
    #[doc = "Bit 12 - Force in TX Stop mode the clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Force in TX Stop mode the data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
    #[doc = "Bit 14 - Contention detection OFF on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<WPCR0rs> {
        CDOFFDL_W::new(self, 14)
    }
    #[doc = "Bit 16 - Turn disable data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn tddl(&mut self) -> TDDL_W<WPCR0rs> {
        TDDL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<WPCR0rs> {
        PDEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Custom time for tCLK"]
    #[inline(always)]
    #[must_use]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<WPCR0rs> {
        TCLKPREPEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Custom time for tCLK"]
    #[inline(always)]
    #[must_use]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<WPCR0rs> {
        TCLKZEROEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<WPCR0rs> {
        THSPREPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<WPCR0rs> {
        THSTRAILEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<WPCR0rs> {
        THSZEROEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<WPCR0rs> {
        TLPXDEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<WPCR0rs> {
        THSEXITEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<WPCR0rs> {
        TLPXCEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Custom time for tCLK"]
    #[inline(always)]
    #[must_use]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<WPCR0rs> {
        TCLKPOSTEN_W::new(self, 27)
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
