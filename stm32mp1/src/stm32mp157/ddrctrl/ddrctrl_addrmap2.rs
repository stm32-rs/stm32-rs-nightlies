#[doc = "Register `DDRCTRL_ADDRMAP2` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP2rs>;
#[doc = "Register `DDRCTRL_ADDRMAP2` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP2rs>;
#[doc = "Field `ADDRMAP_COL_B2` reader - ADDRMAP_COL_B2"]
pub type ADDRMAP_COL_B2_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B2` writer - ADDRMAP_COL_B2"]
pub type ADDRMAP_COL_B2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_COL_B3` reader - ADDRMAP_COL_B3"]
pub type ADDRMAP_COL_B3_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B3` writer - ADDRMAP_COL_B3"]
pub type ADDRMAP_COL_B3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_COL_B4` reader - ADDRMAP_COL_B4"]
pub type ADDRMAP_COL_B4_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B4` writer - ADDRMAP_COL_B4"]
pub type ADDRMAP_COL_B4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_COL_B5` reader - ADDRMAP_COL_B5"]
pub type ADDRMAP_COL_B5_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B5` writer - ADDRMAP_COL_B5"]
pub type ADDRMAP_COL_B5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_COL_B2"]
    #[inline(always)]
    pub fn addrmap_col_b2(&self) -> ADDRMAP_COL_B2_R {
        ADDRMAP_COL_B2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_COL_B3"]
    #[inline(always)]
    pub fn addrmap_col_b3(&self) -> ADDRMAP_COL_B3_R {
        ADDRMAP_COL_B3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_COL_B4"]
    #[inline(always)]
    pub fn addrmap_col_b4(&self) -> ADDRMAP_COL_B4_R {
        ADDRMAP_COL_B4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_COL_B5"]
    #[inline(always)]
    pub fn addrmap_col_b5(&self) -> ADDRMAP_COL_B5_R {
        ADDRMAP_COL_B5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_COL_B2"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b2(&mut self) -> ADDRMAP_COL_B2_W<DDRCTRL_ADDRMAP2rs> {
        ADDRMAP_COL_B2_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - ADDRMAP_COL_B3"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b3(&mut self) -> ADDRMAP_COL_B3_W<DDRCTRL_ADDRMAP2rs> {
        ADDRMAP_COL_B3_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_COL_B4"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b4(&mut self) -> ADDRMAP_COL_B4_W<DDRCTRL_ADDRMAP2rs> {
        ADDRMAP_COL_B4_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - ADDRMAP_COL_B5"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b5(&mut self) -> ADDRMAP_COL_B5_W<DDRCTRL_ADDRMAP2rs> {
        ADDRMAP_COL_B5_W::new(self, 24)
    }
}
#[doc = "DDRCTRL address map register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP2rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap2::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap2::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP2 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP2rs {
    const RESET_VALUE: u32 = 0;
}
