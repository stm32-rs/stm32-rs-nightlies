#[doc = "Register `CREL` reader"]
pub type R = crate::R<CRELrs>;
#[doc = "Field `DAY` reader - DAY"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `MON` reader - MON"]
pub type MON_R = crate::FieldReader;
#[doc = "Field `YEAR` reader - YEAR"]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - SUBSTEP"]
pub type SUBSTEP_R = crate::FieldReader;
#[doc = "Field `STEP` reader - STEP"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `REL` reader - REL"]
pub type REL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DAY"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MON"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - YEAR"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SUBSTEP"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - STEP"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - REL"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRELrs;
impl crate::RegisterSpec for CRELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CRELrs {}
#[doc = "`reset()` method sets CREL to value 0x1111_1111"]
impl crate::Resettable for CRELrs {
    const RESET_VALUE: u32 = 0x1111_1111;
}
