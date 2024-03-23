#[doc = "Register `DDRCTRL_ADDRMAP6` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP6rs>;
#[doc = "Register `DDRCTRL_ADDRMAP6` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP6rs>;
#[doc = "Field `ADDRMAP_ROW_B12` reader - ADDRMAP_ROW_B12"]
pub type ADDRMAP_ROW_B12_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B12` writer - ADDRMAP_ROW_B12"]
pub type ADDRMAP_ROW_B12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_ROW_B13` reader - ADDRMAP_ROW_B13"]
pub type ADDRMAP_ROW_B13_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B13` writer - ADDRMAP_ROW_B13"]
pub type ADDRMAP_ROW_B13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_ROW_B14` reader - ADDRMAP_ROW_B14"]
pub type ADDRMAP_ROW_B14_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B14` writer - ADDRMAP_ROW_B14"]
pub type ADDRMAP_ROW_B14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_ROW_B15` reader - ADDRMAP_ROW_B15"]
pub type ADDRMAP_ROW_B15_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B15` writer - ADDRMAP_ROW_B15"]
pub type ADDRMAP_ROW_B15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPDDR3_6GB_12GB` reader - LPDDR3_6GB_12GB"]
pub type LPDDR3_6GB_12GB_R = crate::BitReader;
#[doc = "Field `LPDDR3_6GB_12GB` writer - LPDDR3_6GB_12GB"]
pub type LPDDR3_6GB_12GB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    pub fn addrmap_row_b12(&self) -> ADDRMAP_ROW_B12_R {
        ADDRMAP_ROW_B12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    pub fn addrmap_row_b13(&self) -> ADDRMAP_ROW_B13_R {
        ADDRMAP_ROW_B13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    pub fn addrmap_row_b14(&self) -> ADDRMAP_ROW_B14_R {
        ADDRMAP_ROW_B14_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    pub fn addrmap_row_b15(&self) -> ADDRMAP_ROW_B15_R {
        ADDRMAP_ROW_B15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&self) -> LPDDR3_6GB_12GB_R {
        LPDDR3_6GB_12GB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b12(&mut self) -> ADDRMAP_ROW_B12_W<DDRCTRL_ADDRMAP6rs> {
        ADDRMAP_ROW_B12_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b13(&mut self) -> ADDRMAP_ROW_B13_W<DDRCTRL_ADDRMAP6rs> {
        ADDRMAP_ROW_B13_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b14(&mut self) -> ADDRMAP_ROW_B14_W<DDRCTRL_ADDRMAP6rs> {
        ADDRMAP_ROW_B14_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b15(&mut self) -> ADDRMAP_ROW_B15_W<DDRCTRL_ADDRMAP6rs> {
        ADDRMAP_ROW_B15_W::new(self, 24)
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr3_6gb_12gb(&mut self) -> LPDDR3_6GB_12GB_W<DDRCTRL_ADDRMAP6rs> {
        LPDDR3_6GB_12GB_W::new(self, 31)
    }
}
#[doc = "DDRCTRL address register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP6rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap6::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP6rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap6::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP6 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP6rs {
    const RESET_VALUE: u32 = 0;
}
